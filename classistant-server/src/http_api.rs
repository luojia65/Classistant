pub mod data;
pub mod users;
pub mod groups;
pub mod sessions;
pub mod forms;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ErrorResponse {
    error_message: String,
}
