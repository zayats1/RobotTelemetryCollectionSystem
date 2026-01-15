use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
fn SelectInfo() -> impl IntoView {
    view! {
        <label for="info_type">Choose which info do you want to visualise:</label>
    < select
    id = "Info" >
        < option
    value = "Moving" > Volvo < / option >
        < option
    value = "Battery" > Saab < / option >
        < option
    value = "Location" > Opel < / option >
        </select>
}
}
