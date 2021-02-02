//! ABI data structure implementation
use macros::response_new;

use crate::*;

impl AppError {
    pub fn new(code: AppErrorCode, message: String) -> Self {
        Self {
            code: code as i32,
            message,
        }
    }
}

impl RequestPing {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

// ResponseMsg new implementation
response_new!(ResponsePong, Pong);
