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
use tokio_util::sync::CancellationToken;
use crate::StreamState;

#[tauri::command(rename_all = "snake_case")]
pub async fn stream_ai_model(
    app_handle: AppHandle,
    state: State<'_, StreamState>,
    content: String,
    model: String,
) -> Result<(), String> {
    // Create a new token and store it
    let token = CancellationToken::new();
    {
        let mut lock = state.cancel_token.lock().await;
        *lock = Some(token.clone());
    }

    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let mut req = GenerationRequest::new(model, content);
    req.think = Some(false);

    let mut stream = ollama
        .generate_stream(req)
        .await
        .map_err(|e| e.to_string())?;

    loop {
        tokio::select! {
            // Check for cancellation
            _ = token.cancelled() => {
                app_handle.emit("ai-stream-cancelled", ()).map_err(|e| e.to_string())?;
                break;
            }
            // Process next chunk
            chunk = stream.next() => {
                match chunk {
                    Some(Ok(responses)) => {
                        for resp in responses {
                            app_handle
                                .emit("ai-stream", resp.response.clone())
                                .map_err(|e| e.to_string())?;

                            // Also check done flag to emit stream end
                            if resp.done {
                                app_handle.emit("ai-stream-done", ()).map_err(|e| e.to_string())?;
                            }
                        }
                    }
                    Some(Err(e)) => return Err(e.to_string()),
                    None => {
                        // Stream finished naturally
                        app_handle.emit("ai-stream-done", ()).map_err(|e| e.to_string())?;
                        break;
                    }
                }
            }
        }
    }

    // Clean up token
    let mut lock = state.cancel_token.lock().await;
    *lock = None;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_stream(state: State<'_, StreamState>) -> Result<(), String> {
    let lock = state.cancel_token.lock().await;
    if let Some(token) = lock.as_ref() {
        token.cancel(); // 🛑 triggers cancellation in stream loop
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stream_ai_thinking_model(
    app_handle: AppHandle,
    content: String,
    model: String,
) -> Result<(), String> {
    let ollama = Ollama::new("http://localhost".to_string(), 11434);

    let mut req = GenerationRequest::new(model, content);

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
