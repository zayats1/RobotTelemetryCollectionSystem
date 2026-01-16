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
    let data:RwSignal<Vec<BatteryInfo>> = RwSignal::new(Vec::new());

    let chart_ref: NodeRef<html::Div> = NodeRef::new();
    let state = use_context::<RwSignal<AppState>>().expect("no state?");
    // Controls whether the chart is visible
    let (show_chart, set_show_chart) = RwSignal::new(false).split();

    let battery_info_fetcher: LocalResource<FetchRes<BatteryInfo>> = LocalResource::new(
          move  || {
          let the_id = state.get().selected_id.unwrap_or_default();
           fetch_battery_info(the_id)
    }
    );
    provide_context(battery_info_fetcher);


    view! {
        <div class="visualizer">
            <div class="buttons">
                <b>"Welcome to visualizer!"</b>
                <article>
                    {move || match state.get().selected_info {
                        Some(info) => {
                            Either::Left(
                                view! {
                                    <h2>"Selected robot:"</h2>
                                    <p>
                                        <b>"id: "</b>
                                        {info.id}
                                    </p>
                                    <p>
                                        <b>"type: "</b>
                                        {info.robot_type.to_string()}
                                    </p>


                                    <div class="controls">
                                        <button on:click=move |_| {
                                            set_show_chart.set(true)
                                        }>"Show Chart"</button>
                                        <button on:click=move |_| {
                                            set_show_chart.set(false)
                                        }>"Hide Chart"</button>
                                    </div>
                                },
                            )
                        }
                        None => Either::Right(view! { <p>"Nothing selected"</p> }),
                    }}
                </article>
            </div>

            <div>
                <Suspense fallback=move || {
                    view! { <p>"Loading Data..."</p> }
                }>
                    {move || {
                        let res = battery_info_fetcher.get();
                        match res {
                            Some(Ok(info)) => {
                                let data_for_chart = info.clone();
                                let data_for_table = info.clone();

                                    view! {
                                        <div class="data-view">

                                            <InfoTable data=data_for_table />

                                              {
                                                if show_chart.get() {
                                                    view! {
                                                        <BatteryChart data=RwSignal::new(
                                                            data_for_chart
                                                        ) />
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }.into_view()
                                            }
                                        </div>
                                    }.into_any()
                            }
                            Some(Err(e)) => {
                                    view! {
                                        <p class="error">{format!("Error fetching data: {}", e)}</p>
                                    }.into_any()
                            }
                            None => view! { <p>"Initializing..."</p> }.into_any(),
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

