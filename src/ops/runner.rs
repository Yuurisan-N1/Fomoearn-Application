use crate::domain::account::Account;
use crate::net::client::Api;
use crate::ops::{auth, user, watch};
use crate::support::cfg::loader::Setup;
use crate::support::error::{refusal, AppError, AppResult};
use crate::support::log::format::{cash, clean_text, countdown, lg, lr, ly, num};

pub async fn run_account(setup: &Setup, index: usize, account: &Account) -> AppResult<()> {
    let proxy = setup.proxy_for(index);
    if let Some(proxy) = proxy {
        ly(&format!("Using proxy {}", proxy.masked()));
    }

    let mut api = Api::open(proxy, account.device_token(), account.android_id())?;

    auth::keyx::perform(&mut api).await?;
    auth::login::perform(&mut api, account).await?;

    let profile = user::dashboard::perform(&api).await?;
    lg(&format!(
        "Signed in to the account as {} with {} USD",
        clean_text(&profile.name, &account.masked()),
        num(profile.balance)
    ));

    auth::session::perform(&mut api).await?;

    loop {
        let task = match watch::list::perform(&api).await {
            Ok(Some(task)) => task,
            Ok(None) => {
                ly("No other video reward was ready for this account");
                break;
            }
            Err(err) => {
                if matches!(err, AppError::Network(_)) {
                    return Err(err);
                }
                lr(&format!(
                    "The video task was refused because {}",
                    refusal(Some(err.to_string()))
                ));
                break;
            }
        };

        countdown(task.seconds, "Next video is ready in").await;

        match watch::serve::perform(&api, &task).await {
            Ok(reward) => {
                lg(&format!(
                    "One video was watched and {} was credited",
                    cash(reward, "USD")
                ));
            }
            Err(err) => {
                if matches!(err, AppError::Network(_)) {
                    return Err(err);
                }
                lr(&format!(
                    "The video claim was refused because {}",
                    refusal(Some(err.to_string()))
                ));
                break;
            }
        }
    }

    Ok(())
}
