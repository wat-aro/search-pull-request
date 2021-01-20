use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Params<'a> {
    pub client_id: &'a str,
    pub device_code: &'a str,
    pub grant_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    error_description: String,
    error: String,
    error_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success(SuccessResponse),
    Err(ErrorResponse),
}
