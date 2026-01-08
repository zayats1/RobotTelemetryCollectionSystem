use leptos::{component, view, IntoView};
use leptos::prelude::*;
#[component]
pub fn AboutPage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <div class="about">
            <h1>"About"</h1>
            <p>"Robot telemetry collection system"</p>
            <p>"Version: 0.1"</p>
        </div>
    }
}