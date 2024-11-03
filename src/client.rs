use std::{
    env, fs,
    process::{exit, Command},
};

use dotenv::dotenv;
use reqwest::blocking::Client;
use toolbox_link_escape::Message;

fn send_to_server(url: String) -> reqwest::Result<()> {
    let client = Client::new();

    let res = client
        .post(format!(
            "http://127.0.0.1:{}/open",
            env::var("TLE_PORT").unwrap_or("8888".to_owned())
        ))
        .json(&Message {
            code: std::env::var("TLE_SECRET").unwrap_or("TLE_TEST".to_owned()),
            link: url,
        })
        .send()?;
    if !res.status().is_success() {
        eprintln!("Result is err ({})", res.status());
        exit(1);
    }
    Ok(())
}

fn open_in_browser(url: String) {
    Command::new(std::env::var("TLE_BROWSER").unwrap_or("firefox".to_owned()))
        .arg(url)
        .output()
        .expect("running firefox failed");
}

fn is_in_toolbox() -> bool {
    fs::exists("/run/.toolboxenv").unwrap_or(false)
}

fn main() -> reqwest::Result<()> {
    dotenv().expect("Can't read env file");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit(1);
    }

    if is_in_toolbox() {
        let res = send_to_server(args[1].clone());
        if res.is_err() {
            eprintln!("Server can't be reached! Trying locally");
            open_in_browser(args[1].clone());
        }
    } else {
        open_in_browser(args[1].clone());
    }

    Ok(())
}
