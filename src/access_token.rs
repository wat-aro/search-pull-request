use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Params<'a> {
    pub client_id: &'a str,
    pub device_code: &'a str,
    pub grant_type: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    error_description: String,
    error: ErrorCode,
    error_uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success(SuccessResponse),
    Err(ErrorResponse),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ErrorCode {
    AuthorizationPending,
    SlowDown,
    ExpiredToken,
    UnsupportedGrantType,
    IncorrectClientCredentials,
    IncorrectDeviceCode,
    AccessDenied,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_params() {
        let params = Params {
            client_id: "client_id",
            device_code: "device_code",
            grant_type: "grant_type",
        };

        if let Ok(json) = serde_json::to_string(&params) {
            assert_eq!(json, "{\"client_id\":\"client_id\",\"device_code\":\"device_code\",\"grant_type\":\"grant_type\"}");
        }
    }

    #[test]
    fn deserialize_error_code() {
        let json = r#"
            [
              "authorization_pending",
              "slow_down",
              "expired_token",
              "unsupported_grant_type",
              "incorrect_client_credentials",
              "incorrect_device_code",
              "access_denied"
            ]
        "#;

        match serde_json::from_str::<Vec<ErrorCode>>(json) {
            Ok(error_code) => {
                assert_eq!(
                    error_code,
                    vec![
                        ErrorCode::AuthorizationPending,
                        ErrorCode::SlowDown,
                        ErrorCode::ExpiredToken,
                        ErrorCode::UnsupportedGrantType,
                        ErrorCode::IncorrectClientCredentials,
                        ErrorCode::IncorrectDeviceCode,
                        ErrorCode::AccessDenied,
                    ]
                );
            }
            Err(err) => panic!(err),
        }
    }
}
