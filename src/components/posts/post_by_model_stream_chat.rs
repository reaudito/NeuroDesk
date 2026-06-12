use crate::components::common::spinner::LoadingSpinner;
use comrak::{
    markdown_to_html_with_plugins, options::Plugins, plugins::syntect::SyntectAdapter, Options,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::{use_clipboard, UseClipboardReturn};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, callback: &js_sys::Function) -> JsValue;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
struct StreamAiArgs {
    content: String,
    model: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputTab {
    Rendered,
    Markdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Role {
    User,
    Assistant,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ChatMessage {
    role: Role,
    content: String,
}

static ADAPTER: OnceLock<SyntectAdapter> = OnceLock::new();

fn render_markdown(markdown: &str) -> String {
    let adapter = ADAPTER.get_or_init(|| SyntectAdapter::new(Some("base16-ocean.dark")));

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(adapter);

    let mut options = Options::default();

    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}

#[component]
pub fn StreamAiModelChatView() -> impl IntoView {
    let (post, set_post) = signal(String::new());
    let (response, set_response) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (models, set_models) = signal::<Vec<ModelData>>(vec![]);
    let (selected_model, set_selected_model) = signal(String::new());
    let (active_tab, set_active_tab) = signal(OutputTab::Rendered);
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![]);

    let UseClipboardReturn {
        is_supported,
        copied,
        copy,
        ..
    } = use_clipboard();

    spawn_local(async move {
        let res = invoke_without_args("list_models").await;
        if let Ok(list) = from_value::<Vec<ModelData>>(res) {
            if let Some(first) = list.first() {
                set_selected_model.set(first.name.clone());
            }
            set_models.set(list);
        }
    });

    spawn_local({
        let set_response = set_response.clone();
        async move {
            let callback = Closure::<dyn FnMut(JsValue)>::new(move |event| {
                if let Some(data) = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                    .ok()
                    .and_then(|v| v.as_string())
                {
                    set_response.update(|r| r.push_str(&data));
                }
            });
            let _ = listen("ai-stream", callback.as_ref().unchecked_ref()).await;
            callback.forget();
        }
    });

    // Listen for cancellation event to reset loading state
    spawn_local({
        async move {
            let callback = Closure::<dyn FnMut(JsValue)>::new(move |_| {
                set_is_loading.set(false);
            });
            let _ = listen("ai-stream-cancelled", callback.as_ref().unchecked_ref()).await;
            callback.forget();
        }
    });

    // When loading finishes, push the completed response into the chat history
    Effect::new(move |prev: Option<bool>| {
        let loading = is_loading.get();
        if prev == Some(true) && !loading {
            let final_response = response.get();
            if !final_response.is_empty() {
                set_messages.update(|m| {
                    m.push(ChatMessage {
                        role: Role::Assistant,
                        content: final_response,
                    });
                });
                set_response.set(String::new());
            }
        }
        loading
    });

    // Core send logic, shared between the Send button and Enter key
    let do_stream = move || {
        let content = post.get();
        let model = selected_model.get();
        if content.is_empty() || model.is_empty() {
            return;
        }

        // Push the user message into chat history immediately
        set_messages.update(|m| {
            m.push(ChatMessage {
                role: Role::User,
                content: content.clone(),
            });
        });
        set_post.set(String::new());

        set_response.set(String::new());
        set_is_loading.set(true);

        spawn_local(async move {
            let args = to_value(&StreamAiArgs { content, model }).unwrap();
            let _ = invoke("stream_ai_model", args).await;
            set_is_loading.set(false);
        });
    };

    let stream_ai = move |_: web_sys::MouseEvent| {
        do_stream();
    };

    // 👇 Stop button handler
    let stop_stream = move |_| {
        spawn_local(async move {
            let _ = invoke_without_args("stop_stream").await;
        });
    };

    let clear = move |_| {
        set_post.set(String::new());
        set_response.set(String::new());
        set_messages.set(vec![]);
    };
    let value = copy.clone();
    view! {
        <div class="flex flex-col h-screen p-4 dark:bg-gray-900 dark:text-white">
            <h2 class="text-xl font-bold mb-2">"AI Streaming Assistant"</h2>

            <div class="flex items-center justify-between mb-2">
                <div class="flex items-center space-x-2">
                    <label>"Model:"</label>
                    <select
                        class="appearance-none p-2 border rounded
                               bg-white text-black
                               dark:bg-gray-800 dark:text-white
                               dark:border-gray-600"
                        on:change=move |ev| set_selected_model.set(event_target_value(&ev))
                    >
                        <For
                            each=move || models.get()
                            key=|m| m.name.clone()
                            children=move |m: ModelData| {
                                view! { <option value=m.name.clone()>{m.name.clone()}</option> }
                            }
                        />
                    </select>
                </div>

                // Tabs (controls rendering style of the live/assistant content)
                <div class="flex border-b dark:border-gray-700">
                    <button
                        class=move || {
                            if active_tab.get() == OutputTab::Rendered {
                                "px-4 py-2 border-b-2 border-blue-500 font-medium"
                            } else {
                                "px-4 py-2 text-gray-500"
                            }
                        }
                        on:click=move |_| set_active_tab.set(OutputTab::Rendered)
                    >
                        "Rendered"
                    </button>

                    <button
                        class=move || {
                            if active_tab.get() == OutputTab::Markdown {
                                "px-4 py-2 border-b-2 border-blue-500 font-medium"
                            } else {
                                "px-4 py-2 text-gray-500"
                            }
                        }
                        on:click=move |_| set_active_tab.set(OutputTab::Markdown)
                    >
                        "Markdown"
                    </button>
                </div>
            </div>

            // 👇 Chat / output area at the top — scrollable, grows to fill space
            <div class="flex-1 overflow-y-auto p-4 border rounded bg-gray-50 dark:bg-gray-800 text-sm space-y-4 mb-4">

                // Render chat history
                <For
                    each=move || messages.get()
                    key=|m| (m.role.clone(), m.content.clone())
                    children=move |m: ChatMessage| {
                        let is_user = m.role == Role::User;
                        let content = m.content.clone();
                        let copy = value.clone();

                        view! {
                            <div class=move || {
                                if is_user {
                                    "ml-auto max-w-[80%] bg-blue-500 text-white rounded-lg px-4 py-2 whitespace-pre-wrap"
                                } else {
                                    "mr-auto max-w-[80%] bg-white dark:bg-gray-700 rounded-lg px-4 py-2 relative group"
                                }
                            }>
                                {move || {
                                    if is_user {
                                        view! { <span class="whitespace-pre-wrap">{content.clone()}</span> }.into_any()
                                    } else {
                                        let content_for_copy = content.clone();
                                        let copy = copy.clone();
                                        view! {
                                            <>
                                                <button
                                                    class="absolute top-1 right-1 px-2 py-1 text-xs rounded bg-gray-200 dark:bg-gray-600
                                                           opacity-0 group-hover:opacity-100 transition-opacity"
                                                    on:click=move |_| copy(&content_for_copy)
                                                >
                                                    {move || if copied.get() { "Copied!" } else { "Copy" }}
                                                </button>
                                                {match active_tab.get() {
                                                    OutputTab::Rendered => {
                                                        let html = render_markdown(&content);
                                                        view! {
                                                            <div
                                                                class="prose dark:prose-invert max-w-none"
                                                                inner_html=html
                                                            />
                                                        }.into_any()
                                                    }
                                                    OutputTab::Markdown => {
                                                        view! {
                                                            <pre class="overflow-x-auto whitespace-pre-wrap break-words text-sm font-mono">
                                                                {content.clone()}
                                                            </pre>
                                                        }.into_any()
                                                    }
                                                }}
                                            </>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        }
                    }
                />

                // Live streaming response (current in-flight assistant message)
                {move || {
                    if is_loading.get() {
                        let copy = copy.clone();
                        view! {
                            <div class="mr-auto max-w-[80%] bg-white dark:bg-gray-700 rounded-lg px-4 py-2 relative group">
                                <LoadingSpinner />
                                <button
                                    class="absolute top-1 right-1 px-2 py-1 text-xs rounded bg-gray-200 dark:bg-gray-600
                                           opacity-0 group-hover:opacity-100 transition-opacity"
                                    on:click=move |_| copy(&response.get())
                                >
                                    {move || if copied.get() { "Copied!" } else { "Copy" }}
                                </button>
                                {move || {
                                    match active_tab.get() {
                                        OutputTab::Rendered => {
                                            let html = render_markdown(&response.get());
                                            view! {
                                                <div
                                                    class="prose dark:prose-invert max-w-none"
                                                    inner_html=html
                                                />
                                            }.into_any()
                                        }
                                        OutputTab::Markdown => {
                                            view! {
                                                <pre class="overflow-x-auto whitespace-pre-wrap break-words text-sm font-mono">
                                                    {response.get()}
                                                </pre>
                                            }.into_any()
                                        }
                                    }
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>

            // 👇 Input area pinned at the bottom
            <div class="flex flex-col space-y-2">
                <textarea
                    class="w-full h-24 p-2 border rounded dark:bg-gray-800"
                    placeholder="Write your prompt here..."
                    prop:value=move || post.get()
                    on:input=move |e| set_post.set(event_target_value(&e))
                    on:keydown=move |e| {
                        if e.key() == "Enter" && !e.shift_key() {
                            e.prevent_default();
                            if !is_loading.get() {
                                do_stream();
                            }
                        }
                    }
                />

                <div class="flex space-x-4">
                    // Start button — hidden while streaming
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded
                               disabled:opacity-50 disabled:cursor-not-allowed"
                        on:click=stream_ai
                        disabled=move || is_loading.get()
                    >
                        "Send"
                    </button>

                    // 👇 Stop button — only visible while streaming
                    {move || is_loading.get().then(|| view! {
                        <button
                            class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded"
                            on:click=stop_stream
                        >
                            "Stop"
                        </button>
                    })}

                    <button
                        class="px-4 py-2 bg-gray-500 text-white rounded"
                        on:click=clear
                    >
                        "Clear"
                    </button>
                </div>
            </div>
        </div>
    }
}
