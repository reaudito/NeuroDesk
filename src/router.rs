use crate::components::posts::list_models::ModelList;
use crate::components::posts::query_ai::QueryAi;
use crate::pages::home::Home;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not Found.">
                <Route path=path!("/") view=Home />
                <Route path=path!("/models") view=ModelList />
                <Route path=path!("/query") view=QueryAi />
            </Routes>
        </Router>
    }
}
