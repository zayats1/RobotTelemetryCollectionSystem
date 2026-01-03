
use leptos::prelude::{Effect, ElementChild, Get, GetUntracked, OnAttribute, RwSignal, Update};
use leptos::{component, island, view, IntoView};
use charming::{component::{Axis, Title}, element::AxisType, series::Line, Chart, EchartsError, HtmlRenderer};
use leptos::html::InnerHtmlAttribute;
use leptos::logging::{error, log};

use leptos::prelude::RenderHtml;
#[island]
pub fn Visualizer() -> impl IntoView{

    let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);


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

        let renderer = HtmlRenderer::new("chart",600, 400);
         let res =  renderer.render( &chart);
        let chart = RwSignal::new(match res {
            Ok(chart) => { chart }
            Err(e) => {error!("{}",e.to_string());
            e.to_string()}
        });



    view! {
          <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js"></script>
         <script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"></script>
        <div>
        <h1>"Welcome to visualizer!"</h1>
            <div inner_html=  chart.get_untracked()> </div>
        </div>
    }


}