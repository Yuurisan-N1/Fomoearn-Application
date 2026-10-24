use crate::net::client::Api;
use crate::support::consts::net::KEYBOX_FILE;
use crate::support::error::{AppError, AppResult};

pub async fn perform(api: &mut Api) -> AppResult<()> {
    let text = std::fs::read_to_string(KEYBOX_FILE)
        .map_err(|_| AppError::config(format!("{} could not be read", KEYBOX_FILE)))?;
    api.attest(&text).await
}
