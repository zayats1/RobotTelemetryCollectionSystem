use charming::{
    component::{Axis, Title}, element::AxisType,
    series::Line,
    Chart,
    WasmRenderer,
};
use leptos::prelude::{
    ClassAttribute, Effect, ElementChild, Get, NodeRef, NodeRefAttribute,
    OnAttribute, RwSignal, Update,
};
use leptos::{html, island, view, IntoView};
use leptos::context::use_context;
use leptos::either::Either;
use leptos::prelude::RenderHtml;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use crate::state::AppState;

#[island]
pub fn Visualizer() -> impl IntoView {
    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);

    let chart_ref: NodeRef<html::Div> = NodeRef::new();
    let state = use_context::<RwSignal<AppState>>().expect("no state?");

    let show_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional cleanup
            div.set_inner_html(r#"<span id="chart"></span>"#);
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

                let renderer = WasmRenderer::new(600, 400);
                renderer.render("chart", &chart).unwrap();
            });

        }
    };

    let hide_chart = move || {
        if let Some(div) = chart_ref.get() {
            div.set_inner_html(""); // optional cleanup
        }
    };

    let Pausable {
        pause,
        resume,
        is_active: _,
    } = use_interval_fn(
        move || {
            data.update(|d| d.rotate_right(1));
        },
        1000,
    );

    view! {
        <div class="visualizer">
            <div class="buttons">
                <button on:click=move |_| show_chart()>"Show Chart"</button>
                <button on:click=move |_| hide_chart()>"Hide Chart"</button>
                <button on:click=move |_| pause()>"Pause"</button>
                <button on:click=move |_| resume()>"Resume"</button>
            </div>
            <div>

                <h1>"Welcome to visualizer!"</h1>
                <div node_ref=chart_ref />
                <article>
                    {match state.get().selected_info {
                        Some(info) => {
                            Either::Left(
                                view! {
                                    <h2>
                                    "Selected robot:"
                                    </h2>
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

