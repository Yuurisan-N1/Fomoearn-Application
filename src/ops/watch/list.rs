use crate::domain::task::Task;
use crate::net::client::Api;
use crate::support::error::AppResult;

pub async fn perform(api: &Api) -> AppResult<Option<Task>> {
    api.next_task().await
}
