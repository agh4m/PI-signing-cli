// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use disa_lib::blockchain::save_certificate;
use disa_lib::communication::{ping, send_file};
use disa_lib::ffi::sig_doc;
use disa_lib::util::{hash_file, save_file, traverse_directory};
use dotenv_codegen::dotenv;
use std::path::Path;
use std::sync::Mutex;
use std::thread::available_parallelism;

struct State {
    token: String,
}

impl State {
    fn new() -> Self {
        Self {
            token: "".to_string(),
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = token.into()
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn login_user(
    state: tauri::State<'_, Mutex<State>>,
    token: String,
) -> Result<String, String> {
    let res = ping(&token).await;

    match res {
        Some(_) => {
            state.lock().unwrap().set_token(&token);
            return Ok("Logged in".to_string());
        }
        None => return Err("Could not login".into()),
    }
}

#[tauri::command]
async fn create_manifest(path: String) -> Result<String, String> {
    println!("{}", path);
    let path = Path::new(&path);
    let save_location = Path::new("/tmp");
    assert_eq!(path.exists(), true);

    let mut documents = Vec::new();
    let threads = available_parallelism().unwrap().get() / 2;

    if path.is_dir() {
        documents = traverse_directory(&path, threads);
    }

    if path.is_file() {
        if let Some(document) = hash_file(&path) {
            documents.push(document);
        } else {
            return Err("Could not hash file")?;
        }
    }

    match save_file(&documents, &save_location.to_str().unwrap()) {
        Some(save_path) => return Ok(save_path),
        None => return Err("Could not write hashes to file")?,
    }
}

#[tauri::command]
async fn sign(hash_json: String) -> Result<String, String> {
    let basic_auth_user = dotenv!("BASIC_AUTH_USER");
    let basic_auth_password = dotenv!("BASIC_AUTH_PASS");
    let application_id = dotenv!("APPLICATION_ID");
    println!("has{}", hash_json);

    let err = sig_doc(
        &hash_json,
        &hash_json.replace(".json", ".asics"),
        true, // sign
        false,
        basic_auth_user,
        basic_auth_password,
        application_id,
    );

    if err != 0 {
        return Err("Could not sign document")?;
    }
    println!("{}", err);

    match hash_file(&Path::new(&hash_json)) {
        Some(document) => {
            return Ok(document.hash);
        }
        None => return Err("Could not hash file")?,
    }
}

#[tauri::command]
async fn blockchain(hashed_manifest: String) -> String {
    let contract_address = dotenv!("CONTRACT_ADDRESS");
    let node_url = dotenv!("NODE_URL");
    let private_key = dotenv!("PRIVATE_KEY");
    let wallet_address = dotenv!("WALLET_ADDRESS");
    println!("{}", contract_address);
    println!("{}", node_url);
    println!("{}", private_key);
    println!("{}", wallet_address);

    let address = save_certificate(
        &hashed_manifest,
        node_url,
        contract_address,
        private_key,
        wallet_address,
    )
    .await
    .unwrap_or_else(|_| "".to_string());
    println!("{}", address);

    return address;
}

#[tauri::command]
async fn server(
    state: tauri::State<'_, Mutex<State>>,
    path: String,
    address: String,
) -> Result<(), String> {
    println!("here");
    let path = Path::new(&path);
    let save_location = Path::new("/tmp");
    let bearer_token = state.lock().unwrap().token.clone();
    println!("{}", bearer_token);

    match send_file(&path, &save_location, &bearer_token, &address).await {
        Ok(_) => return Ok(()),
        Err(res) => return Err(res),
    }
}

fn main() {
    let state = Mutex::new(State::new());
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            login_user,
            create_manifest,
            sign,
            blockchain,
            server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
