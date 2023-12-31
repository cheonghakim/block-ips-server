use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn new(success: bool, message: String, data: Option<T>) -> Self {
        Self {
            success,
            message,
            data,
        }
    }
}
