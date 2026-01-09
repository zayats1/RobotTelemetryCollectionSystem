use leptos::{component, view, IntoView};
use leptos::attr::{alt, src};
use leptos::html::img;
use leptos::prelude::*;


#[component]
pub fn AboutPage() -> impl IntoView {
    // Creates a reactive value to update the button
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);

    let on_click = move |_| *count.write() = (count.get() as u64).wrapping_add(1);
    view! {
        <h1>"About"</h1>
        <div class="about">
            <p>"Robot telemetry collection system"</p>
            <p>"Version: 0.1"</p>
         <button on:click=on_click class="hamster-tap">
         <p> "Tap on the hamster: "</p>
            <img src= "/pictures/hamster.png" width="42pt" height="42pt"  alt="se la vi"/>
        </button>
           <p> "Bucks: " { move || count.get()} "$" </p>
        </div>
    }
}
