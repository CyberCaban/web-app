use serde_json::{json, Value};

#[derive(serde::Serialize)]
pub struct ApiError {
    error_type: String,
    error_msg: String,
}
impl ApiError {
    pub fn unknown_error() -> Self {
        eprintln!("Unknown error");
        ApiError {
            error_type: "Unknown Error".to_string(),
            error_msg: "Unknown Error".to_string(),
        }
    }
    pub fn from_message(message: String) -> Self {
        eprintln!("Error message: {}", message);
        ApiError {
            error_type: "Internal Server Error".to_string(),
            error_msg: message,
        }
    }
    pub fn from_error(error: &impl std::error::Error) -> Self {
        eprintln!("Error: {}", error);
        ApiError {
            error_type: "Internal Server Error".to_string(),
            error_msg: error.to_string(),
        }
    }
    pub fn new(error_type: &str, error_msg: impl ToString) -> Self {
        eprintln!("Error: {} {}", error_type, error_msg.to_string());
        ApiError {
            error_type: error_type.to_string(),
            error_msg: error_msg.to_string(),
        }
    }
    pub fn to_json(&self) -> Value {
        json!(self)
    }
}

#[derive(serde::Serialize)]
enum ApiResponse {
    Ok,
    Err,
}

#[derive(serde::Serialize)]
pub enum RegisterError {
    UserAlreadyExists,
    WeakPassword,
}
impl ToString for RegisterError {
    fn to_string(&self) -> String {
        match self {
            RegisterError::UserAlreadyExists => "UserAlreadyExists".to_string(),
            RegisterError::WeakPassword => "WeakPassword".to_string(),
        }
    }
}
pub enum LoginError {
    UserNotFound,
    WrongPassword,
}
impl ToString for LoginError {
    fn to_string(&self) -> String {
        match self {
            LoginError::UserNotFound => "UserNotFound".to_string(),
            LoginError::WrongPassword => "WrongPassword".to_string(),
        }
    }
}
