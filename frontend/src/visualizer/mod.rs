use leptos::prelude::{OnAttribute, RenderHtml, Write};
use leptos::prelude::{ElementChild, RwSignal};
use leptos::{component, view, IntoView};
use leptos::html::InnerHtmlAttribute;
use plotly::{Plot, Scatter};

#[component]
pub fn Visualizer() -> impl IntoView{
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let id = "plot-div";
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    let layout = plotly::Layout::new().title("Displaying a Chart in Leptos");
    plot.set_layout(layout);

   let chart = plot.to_inline_html(Some(id));
    plot.use_cdn_js();
    
    view! {
        <h1>"Welcome to visualizer!"</h1>
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>

        <button on:click=on_click>"Click Me: " {count}</button>
         <div inner_html = chart/>

    }
}