use crate::modes;
use crate::support::cfg::loader::Setup;
use crate::support::error::AppResult;
use crate::support::log::format::ly;

pub async fn start(setup: &Setup) -> AppResult<()> {
    let picked = std::env::args()
        .nth(1)
        .unwrap_or_default()
        .to_ascii_lowercase();

    match picked.as_str() {
        "" | "run" => modes::run::execute(setup).await,
        "probe" | "check" => modes::probe::execute(setup).await,
        "selftest" | "test" => modes::selftest::execute().await,
        other => {
            ly(&format!("Unknown mode {}", other));
            Ok(())
        }
    }
}
