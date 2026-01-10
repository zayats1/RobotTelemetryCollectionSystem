use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*,  StaticSegment};
use pages::visualizer::Visualizer;
use crate::pages::about::AboutPage;

// Modules
mod components;
mod pages;
pub mod fetcher;
pub mod state;

// Top-Level pages
use crate::pages::home::{HomePage};
use crate::state::AppState;

/// An app router which renders the homepage and handles 404's


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(RwSignal::new(AppState::default()));
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet

        // sets the document title
        <Title text="Welcome to Leptos" />
        <nav>
            <p>"Welcome to Leptos!"</p>
            <a href="/Visualizer/">Visualizer</a>
            <a href="/HomePage/">HomePage</a>
            <a href="/About/">About</a>
        </nav>
        // content for this welcome page
        <Router>
            <main>

                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("HomePage") view=HomePage />
                    <Route path=StaticSegment("About") view=AboutPage />
                    <Route path=StaticSegment("Visualizer") view=Visualizer />
                </Routes>
            </main>
        </Router>
        <footer>"2026"</footer>
    }
}