mod battery;
mod info_table;

use charming::{
    component::{Axis, Title}, element::AxisType,
    series::Line,
    Chart,
    WasmRenderer,
};

use charming::theme::Theme;
use leptos::prelude::{ClassAttribute, Effect, ElementChild, Get, IntoAny, LocalResource, NodeRef, NodeRefAttribute, OnAttribute, RwSignal, Set, Signal, Suspend, Suspense, Write};
use leptos::{html, island, view, IntoView};
use leptos::context::{provide_context, use_context};
use leptos::either::Either;
use leptos::prelude::RenderHtml;

use robot_data::robot_info::BatteryInfo;
use crate::fetcher::{fetch_battery_info, FetchRes};
use crate::pages::visualizer::battery::BatteryChart;
use crate::pages::visualizer::info_table::InfoTable;
use crate::state::AppState;

#[island]
pub fn Visualizer() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().expect("no state?");
    let (show_chart, set_show_chart) = RwSignal::new(true).split(); // Default to true for better UX
    let battery_data = RwSignal::new(Vec::<BatteryInfo>::new());

    let battery_info_fetcher = LocalResource::new(move || {
        let id = state.get().selected_id.unwrap_or_default();
        fetch_battery_info(id)
    });

    view! {
        <div class="dashboard-container">
            // SIDEBAR: Robot Info
            <aside class="sidebar">
                <header>
                    <h2>"Robot Status"</h2>
                    <p class="subtitle">"Telemetry"</p>
                </header>

                {move || match state.get().selected_info {
                    Some(info) => view! {
                        <div class="info-card">
                            <div class="info-group">
                                <label>"ID"</label>
                                <span class="mono">{info.id}</span>
                            </div>
                            <div class="info-group">
                                <label>"TYPE"</label>
                                <span class="badge">{info.robot_type.to_string()}</span>
                            </div>

                            <div class="view-controls">
                                <button
                                    class:active=move || show_chart.get()
                                    on:click=move |_| set_show_chart.set(true)>
                                    "Analytics"
                                </button>
                                <button
                                    class:active=move || !show_chart.get()
                                    on:click=move |_| set_show_chart.set(false)>
                                    "Raw Data"
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    None => view! { <p class="empty-state">"No robot selected"</p> }.into_any(),
                }}
            </aside>

            // MAIN CONTENT
            <main class="content">
                <Suspense fallback=|| view! { <div class="loader">"Fetching data..."</div> }>
                    {move || Suspend::new(async move {
                        if let Ok(inf) = battery_info_fetcher.await {
                            battery_data.set(inf.clone());

                            if show_chart.get() {
                                view! { <BatteryChart data=battery_data.read_only() /> }.into_any()
                            } else {
                                view! { <InfoTable data=inf /> }.into_any()
                            }
                        } else {
                            view! { <div class="error">"Failed to load battery metrics"</div> }.into_any()
                        }
                    })}
                </Suspense>
            </main>
        </div>
    }
}

