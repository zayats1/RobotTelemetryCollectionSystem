use charming::{
    component::{Axis, Title}, element::AxisType,
    series::Line,
    Chart,
    WasmRenderer,
};
use charming::theme::Theme;
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, NodeRef, NodeRefAttribute,
    OnAttribute, RwSignal,
};
use leptos::{html, island, view, IntoView};
use leptos::context::use_context;
use leptos::either::Either;
use leptos::prelude::RenderHtml;
use crate::state::AppState;

#[island]
pub fn Visualizer() -> impl IntoView {
    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);

    let chart_ref: NodeRef<html::Div> = NodeRef::new();
    let state = use_context::<RwSignal<AppState>>().expect("no state?");

    let show_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional cleanup
            div.set_inner_html(r#"<div n class="chart" id="chart"></div>"#);
            // The code should run  on client side only.
            Effect::new(move |_| {
                let local = data.get();

                let chart = Chart::new()
                    .title(Title::new().text("Demo: Leptos + Charming"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(local));

                let renderer = WasmRenderer::new(600, 400).theme(Theme::Dark);
                renderer.render("chart", &chart).unwrap();
            });

        }
    };

    let hide_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional cleanup
        }
    };


    view! {
        <div class="visualizer">
            <div class="buttons">
                <button on:click=move |_| show_chart()>"Show Chart"</button>
                <button on:click=move |_| hide_chart()>"Hide Chart"</button>
            </div>
            <div>

                <h1>"Welcome to visualizer!"</h1>
                <div node_ref=chart_ref />
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
            </div>
        </div>
    }
}

