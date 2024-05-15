// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use disa_lib::communication::ping;
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
    token: String
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

fn main() {
    let state = Mutex::new(State::new());
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![login_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
