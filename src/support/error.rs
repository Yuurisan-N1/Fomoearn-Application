use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Input(String),
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Network(String),
    #[error("{0}")]
    Api(String),
}

impl AppError {
    pub fn input(message: impl Into<String>) -> AppError {
        AppError::Input(message.into())
    }

    pub fn config(message: impl Into<String>) -> AppError {
        AppError::Config(message.into())
    }

    pub fn network(message: impl Into<String>) -> AppError {
        AppError::Network(message.into())
    }

    pub fn api(message: impl Into<String>) -> AppError {
        AppError::Api(message.into())
    }

}

impl AppError {
    pub fn kind(&self) -> &'static str {
        match self {
            AppError::Input(_) => "Input",
            AppError::Config(_) => "Config",
            AppError::Network(_) => "Network",
            AppError::Api(_) => "Api",
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        if err.is_timeout() {
            AppError::Network("the request timed out".to_string())
        } else if err.is_connect() {
            AppError::Network("the connection could not be established".to_string())
        } else if err.is_decode() {
            AppError::Network("the reply could not be read".to_string())
        } else {
            AppError::Network("the request failed".to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> AppError {
        AppError::Network("the reply could not be read".to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

pub fn refusal(code: Option<String>) -> String {
    match code.as_deref() {
        Some("accountLocked") => "the account is locked for now".to_string(),
        Some("invalidCredentials") | Some("wrongPassword") => {
            "a wrong email and password pair".to_string()
        }
        Some("turnstileFailed") | Some("captchaFailed") => {
            "the captcha token was refused".to_string()
        }
        Some("emailNotVerified") => "the email is not verified yet".to_string(),
        Some("rateLimited") | Some("tooManyRequests") => {
            "the server is rate limiting the account".to_string()
        }
        Some("alreadyClaimed") => "the reward was already claimed".to_string(),
        Some("taskExpired") | Some("expired") => "the task expired before the claim".to_string(),
        Some("notEligible") => "the account is not eligible for this task".to_string(),
        Some("sealRequired") | Some("sealInvalid") | Some("sealExpired") | Some("sealVersion") => {
            "the seal header was rejected".to_string()
        }
        Some("vpnBlocked") | Some("proxyBlocked") | Some("vpnDetected") => {
            "the connection looks like a virtual private network".to_string()
        }
        Some("noBonusAvailable") | Some("bonusNotReady") => {
            "the daily bonus is not ready yet".to_string()
        }
        Some("badTask") | Some("taskUnavailable") => {
            "the task is not available any more".to_string()
        }
        Some("notEnoughBalance") => "the balance is not enough for this request".to_string(),
        Some("userNotFound") => "the account was not found on the server".to_string(),
        Some("banned") => "the account is banned from the platform".to_string(),
        _ => "the server refused the request".to_string(),
    }
}
