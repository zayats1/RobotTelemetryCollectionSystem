use std::str::FromStr;
use leptos::{component, view, IntoView};
use leptos::prelude::*;
use robot_data::RobotInfoType;
use crate::state::AppState;

#[component]
pub fn SelectInfo(#[prop(into)] state :RwSignal<AppState>) -> impl IntoView {
    view! {
        <label for="info_type">Choose which info do you want to visualise:</label>
        <select
            id="Info"
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                state.update(|curr| {
                    curr.selected_info_type = RobotInfoType::from_str(&new_value).ok();
                });
            }
        >
            <option value="Noting">Noting</option>
            <option value="Battery">BatteryInfo</option>
            <option value="Movement">Movement</option>
        // <option value="Geodata">Geodata</option>
        </select>
    }
}

