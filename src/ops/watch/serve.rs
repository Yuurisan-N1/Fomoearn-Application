use crate::domain::task::Task;
use crate::net::client::Api;
use crate::support::error::AppResult;

pub async fn perform(api: &Api, task: &Task) -> AppResult<f64> {
    api.claim(task).await
}
