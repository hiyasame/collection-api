use serde::{Serialize, Deserialize};

pub mod upload;
pub mod delete;
pub mod get;
pub mod list;
pub mod random;

#[derive(Serialize, Deserialize)]
struct BaseResponse {
    status: i32,
    message: String,
    data: Option<serde_json::Value>,
}