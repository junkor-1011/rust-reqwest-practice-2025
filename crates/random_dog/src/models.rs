use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Woof {
    pub file_size_bytes: u64,
    #[validate(url(message = "URL is not valid"))]
    pub url: String,
}
