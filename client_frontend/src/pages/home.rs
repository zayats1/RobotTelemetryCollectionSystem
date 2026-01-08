use leptos::{component, view, IntoView};
use leptos::prelude::{GlobalAttributes, OnAttribute, RwSignal, Write};
use leptos_meta::Stylesheet;
use leptos::prelude::*;
use crate::pages::test_gallery::TestGallery;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Stylesheet id="leptos" href="/home_page.css" />
        <button on:click=on_click>"Click Me: " {move || count.get()}</button>
         <TestGallery/>
        <section id="gallery">
        </section>
    }
}