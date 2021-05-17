use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Params<'a> {
    pub client_id: &'a str,
    pub scope: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
}
