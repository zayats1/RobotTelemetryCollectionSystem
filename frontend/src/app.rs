use crate::visualizer::Visualizer;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::home_page::HomePage;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>

            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/telemetry-collector.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <nav>
           <p>"Welcome to Leptos!"</p>
          <a href="/Visualizer/">Visualizer</a>
          <a href="/HomePage/">HomePage</a>
        </nav>
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("HomePage") view=HomePage/>
                    <Route path=StaticSegment("Visualizer") view=Visualizer/>
                </Routes>
            </main>
        </Router>
    }
}

