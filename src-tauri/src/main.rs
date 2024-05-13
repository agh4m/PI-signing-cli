// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use disa_lib::communication::login;
use std::sync::Mutex;

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
    username: String,
    password: String,
) -> Result<String, String> {
    println!("login");
    let token = login(username.to_string(), password.to_string()).await;

    match token {
        Some(t) => {
            state.lock().unwrap().set_token(&t.access_token);
            return Ok("Loged in".to_string());
        }
        None => return Err("Could not login".into()),
    }
}

fn main() {
    let state = Mutex::new(State::new());
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![login_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
