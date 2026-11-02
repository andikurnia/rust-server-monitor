mod api;
mod components;
mod models;
mod pages;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=pages::dashboard::Dashboard />
            </Routes>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
