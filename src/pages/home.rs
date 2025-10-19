// use crate::components::counter_btn::Button;
use crate::components::navigation::nav::Nav;
// use crate::components::posts::create_post::CreatePost;
// use crate::components::posts::post_by_model::CreatePostWithModels;
use crate::components::posts::post_by_model_stream::StreamAiModelView;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Nav />
            <main class="min-h-screen bg-white dark:bg-gray-900 dark:text-white text-gray-900 p-4">
                <section class="text-center py-10">
                    <h1 class="text-4xl font-bold text-purple-600 dark:text-purple-400">
                    "NeuroDesk"
                    </h1>
                    <p class="mt-4 text-lg text-gray-700 dark:text-gray-300">
                        "Ollama Desktop Chat"
                    </p>
                    <p class="mt-2 text-gray-600 dark:text-gray-400">
                        "Chat with Ollama Models"
                    </p>
                </section>



                <StreamAiModelView />

            </main>
        </ErrorBoundary>
    }
}
