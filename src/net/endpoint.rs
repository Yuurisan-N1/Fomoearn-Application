use crate::support::consts::net::{API_BASE, API_ROOT};

pub fn url(path: &str) -> String {
    format!("{}/{}{}", API_BASE, API_ROOT, path.trim_start_matches('/'))
}

pub fn keyx() -> String {
    format!("{}/{}keyx", API_BASE, API_ROOT)
}
