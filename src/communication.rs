use crate::util::Document;
use reqwest;
use serde::Deserialize;
use std::process::exit;

#[derive(Debug, Deserialize)]
struct Response {
    message: String,
}

pub async fn ping_server() -> Result<bool, reqwest::Error> {
    let res = reqwest::get("http://localhost:8000").await?;
    let json: Response = res.json().await?;
    if json.message == "Welcome to DiSA" {
        return Ok(true);
    }
    return Ok(false);
}

pub async fn send_file(_documents: &Vec<Document>, _save_location: &str) {
    let res = ping_server().await;
    if res.is_err() || !res.unwrap() {
        eprintln!("Server is not available, exiting...");
        exit(1);
    }

    println!("Sending file to server");
    todo!();
}
