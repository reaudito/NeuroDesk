use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::LocalModel;
use ollama_rs::Ollama;
use serde_json::{json, Value};
use tauri::AppHandle;
use tauri::Emitter;
use tauri::{Manager, State};
use tauri_plugin_store::StoreExt;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

#[tauri::command(rename_all = "snake_case")]
pub async fn stream_ai_model(
    app_handle: AppHandle,
    content: String,
    model: String,
) -> Result<(), String> {
    let ollama = Ollama::new("http://localhost".to_string(), 11434);

    let mut req = GenerationRequest::new(model, content);
    req.think = Some(false);

    let mut stream = ollama
        .generate_stream(req)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(res) = stream.next().await {
        let responses = res.map_err(|e| e.to_string())?;
        for resp in responses {
            // ✅ send to all webviews (recommended)
            app_handle
                .emit("ai-stream", resp.response.clone())
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
