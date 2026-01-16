mod battery;
mod info_table;

mod select_info;
use charming::theme::Theme;
use leptos::context::{provide_context, use_context};
use leptos::either::Either;
use leptos::prelude::{create_memo, event_target_checked, on_cleanup, set_interval_with_handle, Memo, RenderHtml, StyleAttribute, With};
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, IntoAny, LocalResource, NodeRef, NodeRefAttribute,
    OnAttribute, RwSignal, Set, Signal, Suspend, Suspense, Write,
};
use leptos::{IntoView, html, island, view};

use crate::fetcher::{FetchRes, fetch_battery_info, fetch_movement_info};
use crate::pages::visualizer::battery::BatteryChart;
use crate::pages::visualizer::info_table::InfoTable;
use crate::state::AppState;
use robot_data::RobotInfoType;
use robot_data::robot_info::BatteryInfo;
use crate::pages::visualizer::select_info::SelectInfo;

#[island]
pub fn Visualizer() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().expect("no state?");
    let (show_chart, set_show_chart) = RwSignal::new(true).split(); // Default to true for better UX
    let battery_data = RwSignal::new(Vec::<BatteryInfo>::new());

    let battery_info_fetcher = LocalResource::new(move || {
        let id = state.get().selected_id.unwrap_or_default();
        fetch_battery_info(id)
    });

    let movement_info_fetcher = LocalResource::new(move || {
        let id = state.get().selected_id.unwrap_or_default();
        fetch_movement_info(id)
    });
    // To prevent runtime borrow panic
    let current_info =Memo::new(move |_| {
        state.with(|s| s.selected_info.clone())
    });

    // 1. Create the Auto-Refresh Effect
    let (live_mode, set_live_mode) = RwSignal::new(false).split();

    Effect::new(move |_| {
        if live_mode.get() {
            // Set an interval (e.g., every 5 seconds)
            let handle = set_interval_with_handle(move || {
                // Trigger the resources to fetch again
                battery_info_fetcher.refetch();
                // movement_info_fetcher.refetch();
            }, std::time::Duration::from_secs(5)).expect("failed to set interval");

            // 2. Cleanup: Stop the timer when live_mode is toggled off or component unmounts
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    view! {
        <div class="dashboard-container">
            // SIDEBAR: Robot Info
            <aside class="sidebar">
                <header>
                    <h2>"Robot Status"</h2>
                    <p class="subtitle">"Telemetry"</p>
                </header>
                 <div class="live-toggle-container" style="padding: 10px; background: #1a1a1a;">
                    <label class="switch">
                        <input
                            type="checkbox"
                            on:change=move |ev| set_live_mode.set(event_target_checked(&ev))
                        />
                        <span class="slider"></span>
                        {move || if live_mode.get() { " LIVE MODE ON" } else { " LIVE MODE OFF" }}
                    </label>
                </div>
                  <SelectInfo state=state />
                {move || {

                    match current_info.get() {
                        Some(info) => {
                            let id = info.id.clone();
                            let r_type = info.robot_type.to_string();
                            // Extract the info safely using a memo or direct .with access

                            // Pre-clone ID and Type so the view closures don't fight over 'info'

                            view! {
                                <div class="info-card">
                                    <div class="info-group">
                                        <label>"ID"</label>
                                        <span class="mono">{id}</span>
                                    </div>
                                    <div class="info-group">
                                        <label>"TYPE"</label>
                                        <span class="badge">{r_type}</span>
                                    </div>

                                    <div class="view-controls">
                                        <button
                                            class:active=move || show_chart.get()
                                            on:click=move |_| set_show_chart.set(true)
                                        >
                                            "Analytics"
                                        </button>
                                        <button
                                            class:active=move || !show_chart.get()
                                            on:click=move |_| set_show_chart.set(false)
                                        >
                                            "Raw Data"
                                        </button>
                                    </div>
                                </div>
                                // Pass a clone of the state signal handle, not the value inside it

                            }
                                .into_any()
                        }
                        None => view! { <p class="empty-state">"No robot selected"</p> }.into_any(),
                    }
                }}
            </aside>
            <main class="content">
               { move || {
            if let Some(info_type) = state.get().selected_info_type {
                    match info_type {
                        RobotInfoType::BasicInfo => {

                            view! { <p>"Unimplemented"</p> }
                                .into_any()
                        }
                        RobotInfoType::Geodata => {

                            view! { <p>"Unimplemented"</p> }
                                .into_any()
                        }
                        RobotInfoType::Battery => {
                            view! {
                                <Suspense fallback=|| {
                                    view! { <div class="loader">"Fetching battery data..."</div> }
                                }>
                                    {move || Suspend::new(async move {
                                        if let Ok(inf) = battery_info_fetcher.await {
                                            battery_data.set(inf.clone());
                                            if show_chart.get() {

                                                view! { <BatteryChart data=battery_data.read_only() /> }
                                                    .into_any()
                                            } else {
                                                view! { <InfoTable data=inf /> }.into_any()
                                            }
                                        } else {
                                            view! {
                                                <div class="error">"Failed to load battery metrics"</div>
                                            }
                                                .into_any()
                                        }
                                    })}
                                </Suspense>
                            }
                                .into_any()
                        }
                        RobotInfoType::Movement => {
                            view! {
                                <Suspense fallback=|| {
                                    view! { <div class="loader">"Fetching movement data..."</div> }
                                }>
                                    {move || Suspend::new(async move {
                                        if let Ok(inf) = movement_info_fetcher.await {
                                            view! { <InfoTable data=inf /> }.into_any()
                                        } else {
                                            view! {
                                                <div class="error">"Failed to load movement Info"</div>
                                            }
                                                .into_any()
                                        }
                                    })}
                                </Suspense>
                            }
                                .into_any()
                        }
                    }
                } else {
                    view! { <p>"Noting to show"</p> }.into_any()
                }}
              }
            </main>
        </div>
    }
}
