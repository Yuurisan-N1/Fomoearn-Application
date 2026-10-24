mod app;
mod domain;
mod modes;
mod net;
mod ops;
mod support;

use std::process::ExitCode;

use app::bootstrap;
use support::banner::print_banner;
use support::log::format::{finish_status, lr};

#[tokio::main]
async fn main() -> ExitCode {
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        finish_status();
        lr("Script stopped by user");
        std::process::exit(0);
    });

    support::log::init::arm();
    print_banner();

    let setup = match bootstrap::boot() {
        Ok(setup) => setup,
        Err(err) => {
            lr(&format!("The program stopped: {}", err));
            return ExitCode::FAILURE;
        }
    };

    match app::dispatch::start(&setup).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            lr(&format!("The program stopped: {}", err));
            ExitCode::FAILURE
        }
    }
}
