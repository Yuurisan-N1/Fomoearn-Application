use crate::domain::account::Account;
use crate::net::client::Api;
use crate::support::error::AppResult;

pub async fn perform(api: &mut Api, account: &Account) -> AppResult<()> {
    api.sign_in(&account.email, &account.password).await
}
