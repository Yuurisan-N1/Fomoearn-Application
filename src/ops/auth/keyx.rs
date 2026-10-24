use crate::net::client::Api;
use crate::support::error::AppResult;

pub async fn perform(api: &mut Api) -> AppResult<()> {
    api.keyx().await
}
