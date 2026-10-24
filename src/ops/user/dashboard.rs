use crate::net::client::{Api, Profile};
use crate::support::error::AppResult;

pub async fn perform(api: &Api) -> AppResult<Profile> {
    api.profile().await
}
