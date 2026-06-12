use crate::components::navigation::nav::Nav;
use crate::components::posts::post_by_model_stream_chat::StreamAiModelChatView;
use leptos::prelude::*;
#[component]
pub fn StreamAiChat() -> impl IntoView {
    view! {
        <>
        <Nav />
        <StreamAiModelChatView />
        </>
    }
}
