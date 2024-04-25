use crate::util::create_tar;
use crate::util::Document;
use reqwest::{get, multipart, Client, Error};
use serde::Deserialize;
use std::fs::read;
use std::path::Path;
use std::process::exit;

#[derive(Debug, Deserialize)]
struct Response {
    message: String,
}

#[derive(Debug, Deserialize)]
struct FileResponse {
    message: String,
    uuid: String,
}

// #{derive(Debug, Serialize, Deserialize)]
// struct FileBody {
//     manifest_hash
// }

pub async fn ping_server() -> Result<bool, Error> {
    let res = get("http://localhost:8000").await?;
    let json: Response = res.json().await?;
    if json.message == "Welcome to DiSA" {
        return Ok(true);
    }

    // This is kinda cursed, but should also never happen
    return Ok(false);
}

pub async fn send_file(path: &Path, save_location: &Path, token: &str) {
    let res = ping_server().await;
    if res.is_err() || !res.unwrap() {
        eprintln!("Server is not available, exiting...");
        exit(1);
    }

    let archive = create_tar(&path, &save_location);

    println!("Sending file to server");

    if archive.is_none() {
        eprintln!("Failed to create archive");
        exit(1);
    }

    let read_archive = read(&archive.unwrap()).unwrap();
    let part = multipart::Part::bytes(read_archive);
    // this looks wierd, but I did not feel like dealing with lifetimes
    let file = multipart::Form::new().part(path.to_str().unwrap().to_string(), part);

    // Send the file to the Server
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/collections")
        .bearer_auth(token.to_string())
        .multipart(file)
        .send()
        .await;

    match response {
        Ok(res) => match res.status().as_u16() {
            401 => println!("Failed to authenticate"),
            _=> {
                println!("File sent successfully");
                // let json: FileResponse = res.json().await.unwrap();
                // println!("UUID: {}", json.uuid);
            }
        },
        Err(res) => {
            eprintln!("Failed to send file");
            eprintln!("{}", res.to_string());
        }
    }
}
