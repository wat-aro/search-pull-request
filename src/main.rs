use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

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
    Ok(())
}
