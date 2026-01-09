use std::sync::Arc;
use leptos::{component, view, IntoView};
use leptos::prelude::{GlobalAttributes, OnAttribute, RwSignal, Write};
use leptos_meta::Stylesheet;
use leptos::prelude::*;
use leptos::svg::view;
use leptos::tachys::view;
use crate::app_model::{fetch_robots, AppModel};

use crate::pages::test_gallery::TestGallery;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);


    let on_click = move |_| *count.write() += 1;

    let model = use_context::<RwSignal<AppModel>>().expect("no model?");
    let robots = LocalResource::new(  || {
       fetch_robots()});
    view! {
        <Stylesheet id="leptos" href="/home_page.css" />
        <button on:click=on_click>"Click Me: " {move || count.get()}</button>
        <section id="gallery">
   <Suspense fallback=|| view! { <p>"Loading..."</p> }>
    {move || {
        robots
            .read().iter()
            .map(|res| {
               format!("{:?}",*res)
            }).collect_view()
    }}
</Suspense>
         // <TestGallery/> // for test
        </section>
    }
}

