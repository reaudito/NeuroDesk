use crate::components::navigation::nav::Nav;
use crate::components::posts::thinking_post_by_model_stream::StreamAiThinkingView;
use leptos::prelude::*;
#[component]
pub fn StreamAiThinking() -> impl IntoView {
    view! {
        <>
        <Nav />
        <StreamAiThinkingView />
        </>
    }
}
