use crate::components::navigation::nav::Nav;
use crate::components::posts::post_by_model::CreatePostWithModels;
use leptos::prelude::*;
#[component]
pub fn QueryAi() -> impl IntoView {
    view! {
        <>
        <Nav />
        <CreatePostWithModels />
        </>
    }
}
