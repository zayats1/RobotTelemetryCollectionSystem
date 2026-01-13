use charming::{
    component::{Axis, Title}, element::AxisType,
    series::Line,
    Chart,
    WasmRenderer,
};

use charming::theme::Theme;
use leptos::prelude::{ClassAttribute, Effect, ElementChild, Get, LocalResource, NodeRef, NodeRefAttribute, OnAttribute, Resource, RwSignal, Suspend, Suspense, Write};
use leptos::{html, island, view, IntoView};
use leptos::context::use_context;
use leptos::either::Either;
use leptos::prelude::RenderHtml;

use robot_data::robot_info::BatteryInfo;
use crate::fetcher::{fetch_battery_info, FetchRes};
use crate::state::AppState;

#[island]
pub fn Visualizer() -> impl IntoView {
    let data:RwSignal<Vec<BatteryInfo>> = RwSignal::new(Vec::new());

    let chart_ref: NodeRef<html::Div> = NodeRef::new();
    let state = use_context::<RwSignal<AppState>>().expect("no state?");


    let battery_info_fetcher: LocalResource<FetchRes<BatteryInfo>> = LocalResource::new(move || {
          let the_id = state.get().selected_id.unwrap_or_default();
           fetch_battery_info(the_id)

    });
     // Todo: only for battery info for now
    let show_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional clean-up
            div.set_inner_html(r#"<div n class="chart" id="chart"></div>"#);
            // The code should run  on client side only.
            Effect::new(move |_| {
                let local = data.get();
                let mut time = Vec::new();
                let mut health = Vec::new();
                let mut capacity = Vec::new();
                 local.iter().for_each(|val| {
                     time.push(val.timestamp.time().to_string());
                     health.push(val.health);
                     capacity.push(val.capacity/val.total_capacity);


                 });
                let chart = Chart::new()
                    .title(Title::new().text("Demo: Leptos + Charming"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(time),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(health.clone()))
                    .series(Line::new().data(capacity.clone()));

                let renderer = WasmRenderer::new(600, 400).theme(Theme::Dark);
                renderer.render("chart", &chart).unwrap();
            });
        }
    };;

    let hide_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional clean-up
        }
    };


    view! {
        <div class="visualizer">
            <div class="buttons">
                <b>"Welcome to visualizer!"</b>
                <button on:click=move |_| show_chart()>"Show Chart"</button>
                <button on:click=move |_| hide_chart()>"Hide Chart"</button>
            </div>
            <div>
                    <article>
                    {match state.get().selected_info {
                        Some(info) => {
                            Either::Left(
                                view! {
                                    <h2>"Selected robot:"</h2>
                                    <p>
                                        <b>"id:"</b>
                                        {info.id}
                                    </p>
                                    <p>
                                        <b>"type:"</b>
                                        {info.robot_type.to_string()}
                                    </p>
                                },
                            )
                        }
                        None => Either::Right(view! { <p>Nothing selected</p> }),
                    }}
                </article>

                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {Suspend::new(async move {
                        let res = battery_info_fetcher.await;
                        match res {
                            Ok(inf) => {
                                let mut table = tabled::Table::new(&inf);
                                table.with(tabled::settings::Style::psql());
                                *data.write() = inf.clone();
                                Either::Left(

                                    view! {
                                        <div class="info_table">
                                            <pre>{table.to_string()}</pre>
                                        </div>
                                    },
                                )
                            }
                            Err(e) => {
                                Either::Right(

                                    view! { <p>{format!("Error: {}", e)}</p> },
                                )
                            }
                        }
                    })}
                </Suspense>
                <div node_ref=chart_ref />


                <label for="info_type">Choose which info do you want to visualise:</label>

                <select id="cars">
                    <option value="volvo">Volvo</option>
                    <option value="saab">Saab</option>
                    <option value="opel">Opel</option>
                    <option value="audi">Audi</option>
                </select>

            </div>
        </div>
    }
}

