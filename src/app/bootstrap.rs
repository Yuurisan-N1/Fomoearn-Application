use std::path::PathBuf;

use crate::support::cfg::loader::{self, Setup};
use crate::support::error::AppResult;

pub fn boot() -> AppResult<Setup> {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loader::load(&root)
}
