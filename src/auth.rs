use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use std::{fs, io::Write, thread, time::Duration};

// maybe handle the expiry possibility???
#[derive(Deserialize, Debug)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    // expires_in: u64,
    interval: u64,
}

#[derive(Deserialize, Debug)]
struct PollResponse {
    access_token: Option<String>,
    // could get the type of error here
    // token_type: Option<String>,
    // scope: Option<String>,
    // error: Option<String>,
    // error_description: Option<String>,
}

/// Device flow
/// Allows you to authorize users for a headless application such as a CLI tool
/// https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
fn device_flow_authentication() -> Result<String> {
    let client = reqwest::blocking::Client::new();

    // Step 1: App requests the device and user verification codes from Github
    let resp = client
        .post("https://github.com/login/device/code")
        .query(&[("client_id", "Ov23li900M86x5s8Q6iU"), ("scope", "project")])
        .header("Accept", "application/json")
        .header("User-Agent", "motoro")
        .send()?;

    let data = resp.json::<DeviceCodeResponse>()?;

    // Step 2: Prompt the user to enter the user code in a browser
    println!(
        "Paste this code into the browser window: {}",
        data.user_code.magenta().bold()
    );
    opener::open(data.verification_uri)?;

    // Step 3: App polls GitHub to check if the user authorized the device
    // This is not complete or correct error handling
    let mut attempts = 0;
    let max_attempts = 120;
    let token = loop {
        thread::sleep(Duration::from_secs(data.interval));
        attempts += 1;

        if attempts > max_attempts {
            return Err(anyhow::anyhow!(
                "Authentication timed out. Please try again."
            ));
        }

        let poll_resp = client
            .post("https://github.com/login/oauth/access_token")
            .query(&[
                ("client_id", "Ov23li900M86x5s8Q6iU"),
                ("device_code", &data.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .header("Accept", "application/json")
            .header("User-Agent", "motoro")
            .send()?;

        let poll_data = poll_resp.json::<PollResponse>()?;

        if let Some(token) = poll_data.access_token {
            break token;
        }
        // else {
        //     println!("{}", poll_data.error.unwrap())
        // }
    };

    // Now the app has an access token that can be used to make requests to the github API on behalf of user
    Ok(token)
}

/// Check for token on disk otherwise use device flow authentication
pub fn get_access_token() -> Result<String> {
    let path = dirs::config_dir().unwrap().join("motoro.txt");

    match fs::read_to_string(&path) {
        Ok(token) => {
            // println!("Using saved token!: {}", token);
            Ok(token)
        }
        Err(_) => {
            let token = device_flow_authentication()?;
            let mut handle = fs::File::create(&path)?;
            write!(&mut handle, "{}", token)?;

            Ok(token)
        }
    }
}
