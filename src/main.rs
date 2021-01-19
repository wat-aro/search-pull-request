use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

#[derive(Debug, Serialize)]
struct DeviceCodeParams<'a> {
    client_id: &'a str,
    scope: &'a str,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Debug, Serialize)]
struct AccessTokenParams<'a> {
    client_id: &'a str,
    device_code: &'a str,
    grant_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenErrorResponse {
    error_description: String,
    error: String,
    error_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum AccessTokenResponseType {
    Ok(AccessTokenResponse),
    Err(AccessTokenErrorResponse),
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client_id = "2db88301ea022dd5bc00";
    let device_code_params = DeviceCodeParams {
        client_id,
        scope: "repo",
    };

    let request = reqwest::Client::new()
        .post("https://github.com/login/device/code")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .json(&device_code_params);

    let device_code_response: DeviceCodeResponse = request.send().await?.json().await?;

    println!(
        "{} First copy your one-time code: {}",
        "!".yellow(),
        device_code_response.user_code
    );
    print!(
        "{} to open github.com in your browser... ",
        "Press Enter".bold()
    );
    stdout().flush().unwrap();
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();

    let launcher = env::var("BROWSER").unwrap_or(String::from("open"));
    Command::new(launcher)
        .args(&[device_code_response.verification_uri])
        .output()
        .expect("Failed to open browser");

    let grant_type = "urn:ietf:params:oauth:grant-type:device_code";
    let access_token_params = AccessTokenParams {
        client_id,
        device_code: &device_code_response.device_code,
        grant_type,
    };

    let request = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .json(&access_token_params);

    let response = request
        .send()
        .await?
        .json::<AccessTokenResponseType>()
        .await?;

    match response {
        AccessTokenResponseType::Ok(success) => println!("{:#?}", success),
        AccessTokenResponseType::Err(error) => println!("{:#?}", error),
    };

    Ok(())
}
