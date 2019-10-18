pub mod users;
pub mod data;
pub mod groups;
pub mod sessions;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ErrorResponse {
    error_message: String,
}
