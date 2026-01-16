use leptos::prelude::*;

#[component]
pub fn LiveToggle(set_live_mode: WriteSignal<bool>, live_mode: Signal<bool>) -> impl IntoView {
    view! {
        <div class="live-toggle-container" style="padding: 10px; background: #1a1a1a;">
            <label class="switch">
                <input type="checkbox" on:change=move |ev| set_live_mode.set(event_target_checked(&ev)) />
                <span class="slider"></span>
                {move || if live_mode.get() { " LIVE MODE ON" } else { " LIVE MODE OFF" }}
            </label>
        </div>
    }
}