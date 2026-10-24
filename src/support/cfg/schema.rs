use std::path::Path;

use serde_json::Value;

use crate::support::error::{AppError, AppResult};

const DEFAULTS: &str = "{\n  \"settings\": {\n    \"sleep_seconds\": 3600\n  }\n}\n";

#[derive(Debug, Clone)]
pub struct Config {
    value: Value,
}

impl Config {
    pub fn defaults() -> Config {
        Config {
            value: serde_json::from_str(DEFAULTS).unwrap_or(Value::Null),
        }
    }

    pub fn parse(text: &str) -> AppResult<Config> {
        let value: Value = serde_json::from_str(text)
            .map_err(|err| AppError::config(format!("config.json is not valid json: {}", err)))?;
        let config = Config { value };
        config.validate()?;
        Ok(config)
    }

    pub fn load(path: &Path) -> AppResult<Config> {
        match std::fs::read_to_string(path) {
            Ok(text) => Config::parse(&text),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                std::fs::write(path, DEFAULTS).map_err(|err| {
                    AppError::config(format!("config.json could not be written: {}", err))
                })?;
                Ok(Config::defaults())
            }
            Err(err) => Err(AppError::config(format!(
                "config.json could not be read: {}",
                err
            ))),
        }
    }

    pub const DEFAULT_SLEEP_SECONDS: u64 = 3600;

    pub fn sleep_seconds(&self) -> u64 {
        let raw = self
            .pick_u64("sleep_seconds")
            .unwrap_or(Config::DEFAULT_SLEEP_SECONDS);
        raw.clamp(5, 86_400)
    }

    fn settings(&self) -> Option<&Value> {
        match self.value.get("settings") {
            Some(value) if value.is_object() => Some(value),
            _ => None,
        }
    }

    fn pick_u64(&self, key: &str) -> Option<u64> {
        let settings = self.settings()?;
        let found = settings.get(key)?;
        if let Some(raw) = found.as_u64() {
            return Some(raw);
        }
        found.as_str().and_then(|raw| raw.trim().parse().ok())
    }

    fn validate(&self) -> AppResult<()> {
        if self.value.get("settings").is_none() {
            return Err(AppError::config("config.json needs a settings block"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_canon_block() {
        assert_eq!(DEFAULTS.len(), 50);
        assert_eq!(DEFAULTS.lines().count(), 5);
        let config = Config::parse(DEFAULTS).unwrap();
        assert_eq!(config.sleep_seconds(), Config::DEFAULT_SLEEP_SECONDS);
    }

    #[test]
    fn sleep_is_clamped() {
        let config = Config::parse(r#"{"settings":{"sleep_seconds":1}}"#).unwrap();
        assert_eq!(config.sleep_seconds(), 5);
        let config = Config::parse(r#"{"settings":{"sleep_seconds":99999999}}"#).unwrap();
        assert_eq!(config.sleep_seconds(), 86_400);
    }

    #[test]
    fn missing_settings_block_is_rejected() {
        assert!(Config::parse("{}").is_err());
    }
}
