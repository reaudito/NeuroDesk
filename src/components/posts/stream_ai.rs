use crate::components::navigation::nav::Nav;
use crate::components::posts::post_by_model_stream::StreamAiModelView;
use leptos::prelude::*;
#[component]
pub fn StreamAi() -> impl IntoView {
    view! {
        <>
        <Nav />
        <StreamAiModelView />
        </>
    }
}
