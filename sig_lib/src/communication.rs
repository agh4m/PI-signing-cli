use crate::util::create_tar;
use reqwest::{get, multipart, Client, Error};
use serde::Deserialize;
use std::fmt::Display;
use std::fs::read;
use std::path::Path;
use std::process::exit;

#[derive(Debug, Deserialize)]
struct Response {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct FileResponse {
    pub message: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

impl Display for LoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

pub async fn ping_server() -> Result<bool, Error> {
    let res = get("http://localhost:8000").await?;
    let json: Response = res.json().await?;
    if json.message == "Welcome to DiSA" {
        return Ok(true);
    }

    // This is kinda cursed, but should also never happen
    return Ok(false);
}

pub async fn send_file(
    path: &Path,
    save_location: &Path,
    token: &str,
    address: &str,
) -> Result<(), String> {
    let res = ping_server().await;
    if res.is_err() || !res.unwrap() {
        eprintln!("Server is not available, exiting...");
        exit(1);
    }

    let archive = create_tar(&path, &save_location).unwrap();

    println!("Sending file to server");

    let read_archive = read(&archive).unwrap();

    let part = multipart::Part::bytes(read_archive)
        .file_name(archive)
        .mime_str("application/x-tar")
        .unwrap();

    let address_part = multipart::Part::text(address.to_string());

    // this looks wierd, but I did not feel like dealing with lifetimes
    let file = multipart::Form::new()
        // .part("manifest_hash", man_hash_part)
        .part("transaction_address", address_part)
        .part("file", part);

    // Send the file to the Server
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/collections/")
        .bearer_auth(token.to_string())
        .multipart(file)
        .send()
        .await;

    match response {
        Ok(res) => match res.status().as_u16() {
            200 => return Ok(()),
            _ => return Err(res.status().to_string()),
        },
        Err(res) => return Err(res.to_string()),
    }
}

pub async fn login(username: String, password: String) -> Option<LoginResponse> {
    let username_part = multipart::Part::text(username);
    let password_part = multipart::Part::text(password);
    let form = multipart::Form::new()
        .part("username".to_string(), username_part)
        .part("password".to_string(), password_part);

    let client = Client::new();
    let response = client
        .post("http://localhost:8000/users/login/")
        .multipart(form)
        .send()
        .await;

    match response {
        Ok(res) => match res.status().as_u16() {
            200 => {
                return Some(res.json::<LoginResponse>().await.unwrap());
            }
            _ => {
                println!("{}", res.text().await.unwrap());
                return None;
            }
        },
        Err(_) => {
            return None;
        }
    }
}

pub async fn ping(token: &str) -> Option<&str> {
    let client = Client::new();
    let response = client
        .get("http://localhost:8000/ping/")
        .bearer_auth(token.to_string())
        .send()
        .await;

    match response {
        Ok(res) => match res.status().as_u16() {
            200 => {
                assert_eq!(res.text().await.unwrap(), "\"pong\"");
                return Some("Pong");
            }
            _ => return None,
        },
        _ => return None,
    }
}
