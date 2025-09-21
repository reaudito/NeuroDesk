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
            </Routes>
        </Router>
    }
}
