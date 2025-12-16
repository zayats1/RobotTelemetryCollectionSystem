use leptos::prelude::{OnAttribute, Write};
use leptos::prelude::{ElementChild, RwSignal};
use leptos::{component, view, IntoView};


#[component]
pub fn RobotMap() -> impl IntoView{
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    view! {
        <h1>"Welcome to visualizer!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}