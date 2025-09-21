use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::LocalModel;
use ollama_rs::Ollama;
use serde_json::{json, Value};
use tauri_plugin_store::StoreExt;
pub mod types;
pub use types::*;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn ask_ai(content: String) -> String {
    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "qwen3:latest".to_string();
    // let model = "deepseek-r1:1.5b".to_string();
    let mut req = GenerationRequest::new(model, content);
    req.think = Some(false);

    match ollama.generate(req).await {
        Ok(res) => res.response,
        Err(e) => format!("Error: {}", e),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn list_models() -> Vec<ModelData> {
    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let res = ollama.list_local_models().await.unwrap();

    let model_data = res
        .into_iter()
        .map(|model| ModelData {
            name: model.name,
            modified_at: model.modified_at,
            size: model.size,
        })
        .collect();

    model_data
}

#[tauri::command(rename_all = "snake_case")]
async fn ask_ai_model(content: String, model: String) -> String {
    let ollama = Ollama::new("http://localhost".to_string(), 11434);

    let mut req = GenerationRequest::new(model, content);
    req.think = Some(false);

    match ollama.generate(req).await {
        Ok(res) => res.response,
        Err(e) => format!("Error: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            ask_ai,
            list_models,
            ask_ai_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
