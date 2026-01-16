use charming::{Chart, WasmRenderer};
use charming::component::{Axis, Title};
use charming::element::AxisType;
use charming::series::Line;
use charming::theme::Theme;
use leptos::component;
use leptos::logging::error;
use leptos::prelude::*;
use robot_data::robot_info::BatteryInfo;

#[component]
pub fn BatteryChart(
    // We use a closure/signal accessor so the effect can react to changes.
    #[prop(into)]
    data: Signal<Vec<BatteryInfo>>,
) -> impl IntoView {

    let chart_id = "chart";

    Effect::new(move |_| {
        // 1. Get the data. This subscribes the effect to updates.
        let info = data.get();
        if info.is_empty() { return; }

        // 2. Prepare data vectors
        let mut time = Vec::new();
        let mut health = Vec::new();
        let mut capacity = Vec::new();

        for val in info.iter() {
            time.push(val.timestamp.time().to_string());
            health.push(val.health);
            // Protect against division by zero if needed
            let cap_ratio = if val.total_capacity != 0.0 {
                val.capacity / val.total_capacity
            } else {
                0.0
            };
            capacity.push(cap_ratio);
        }

        // 3. Configure Chart
        let chart = Chart::new()
            .title(Title::new().text("Battery Metrics"))
            .x_axis(Axis::new().type_(AxisType::Category).data(time))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().name("Health").data(health))
            .series(Line::new().name("Capacity").data(capacity));

        // 4. Render
        // Note: We don't need manual DOM cleanup; if data changes,
        // Charming usually overwrites, or the component remounts.
        request_animation_frame(move || {
            let renderer = WasmRenderer::new(600, 400).theme(Theme::Dark);
            // We ignore the result here, but in production, you might want to log errors
            let err = renderer.render(chart_id, &chart);
            if let Err(e) = err {
                error!("{}", e.to_string());
            }
        })
    });

    view! { <div id=chart_id class="chart"></div> }
}


