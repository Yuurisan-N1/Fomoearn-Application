use crate::net::client::Api;
use crate::ops::{auth, user};
use crate::support::cfg::loader::Setup;
use crate::support::error::{AppError, AppResult};
use crate::support::log::format::{clean_text, lg, ly, num};

pub async fn execute(setup: &Setup) -> AppResult<()> {
    let account = setup
        .accounts
        .first()
        .ok_or_else(|| AppError::input("data.txt holds no account"))?;

    lg("Probe mode runs read only so nothing is claimed");

    let proxy = setup.proxy_for(0);
    if let Some(proxy) = proxy {
        ly(&format!("Using proxy {}", proxy.masked()));
    }

    let mut api = Api::open(proxy, account.device_token(), account.android_id())?;

    auth::keyx::perform(&mut api).await?;
    lg("The transport handshake was accepted by the server");

    auth::login::perform(&mut api, account).await?;
    lg("The account password was accepted by the server");

    let profile = user::dashboard::perform(&api).await?;
    lg(&format!(
        "Signed in to the account as {} with {} USD",
        clean_text(&profile.name, &account.masked()),
        num(profile.balance)
    ));
    ly(&format!(
        "The account watched {} out of {} videos today",
        profile.watched, profile.limit_day
    ));

    auth::session::perform(&mut api).await?;
    lg("The attestation statement was accepted by the server");

    Ok(())
}
