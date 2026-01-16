mod battery;
mod info_table;
mod select_info;
mod sidebar;
mod live_toggle;

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
use crate::pages::visualizer::live_toggle::LiveToggle;
use crate::pages::visualizer::select_info::SelectInfo;
use crate::pages::visualizer::sidebar::Sidebar;

#[island]
pub fn Visualizer() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().expect("no state?");
    let (show_chart, set_show_chart) = RwSignal::new(true).split();
    let (live_mode, set_live_mode) = RwSignal::new(false).split();

    // Resources
    let battery_info_fetcher = LocalResource::new(move || {
        let id = state.get().selected_id.unwrap_or_default();
        fetch_battery_info(id)
    });

    let movement_info_fetcher = LocalResource::new(move || {
        let id = state.get().selected_id.unwrap_or_default();
        fetch_movement_info(id)
    });

    // Memos
    let current_info = Memo::new(move |_| state.with(|s| s.selected_info.clone()));

    // Auto-refresh Effect
    Effect::new(move |_| {
        if live_mode.get() {
            let handle = set_interval_with_handle(move || {
                battery_info_fetcher.refetch();
                movement_info_fetcher.refetch();
            }, std::time::Duration::from_secs(5)).ok();

            on_cleanup(move || { if let Some(h) = handle { h.clear(); } });
        }
    });

    view! {
        <div class="dashboard-container">
            <aside class="sidebar">
                <header>
                    <h2>"Robot Status"</h2>
                    <p class="subtitle">"Telemetry"</p>
                </header>
                <LiveToggle set_live_mode=set_live_mode live_mode=live_mode.into() />
                <SelectInfo state=state />
                <Sidebar current_info=current_info show_chart=show_chart.into() set_show_chart=set_show_chart />
            </aside>

            <main class="content">
                {move || {
                    let info_type = state.get().selected_info_type;
                    match info_type {
                        Some(RobotInfoType::Battery) => view! {
                            <Suspense fallback=|| view! { <div class="loader">"Fetching battery..."</div> }>
                                {move || Suspend::new(async move {
                                    match battery_info_fetcher.await {
                                        Ok(inf) if show_chart.get() => view! { <BatteryChart data=Signal::derive(move || inf.clone()) /> }.into_any(),
                                        Ok(inf) => view! { <InfoTable data=inf /> }.into_any(),
                                        Err(_) => view! { <div class="error">"Failed to load metrics"</div> }.into_any(),
                                    }
                                })}
                            </Suspense>
                        }.into_any(),
                        Some(RobotInfoType::Movement) => view! {
                            <Suspense fallback=|| view! { <div class="loader">"Fetching movement..."</div> }>
                                {move || Suspend::new(async move {
                                    match movement_info_fetcher.await {
                                        Ok(inf) => view! { <InfoTable data=inf /> }.into_any(),
                                        Err(_) => view! { <div class="error">"Error loading movement"</div> }.into_any(),
                                    }
                                })}
                            </Suspense>
                        }.into_any(),
                        _ => view! { <p>"Nothing to show"</p> }.into_any(),
                    }
                }}
            </main>
        </div>
    }
}
