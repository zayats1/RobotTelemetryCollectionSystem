use leptos::prelude::*;
use robot_data::robot_info::BasicInfo;

#[component]
pub fn Sidebar(
    current_info: Memo<Option<BasicInfo>>,
    show_chart: Signal<bool>,
    set_show_chart: WriteSignal<bool>
) -> impl IntoView {
    view! {
        {move || match current_info.get() {
            Some(info) => {
                let id = info.id.clone();
                let r_type = info.robot_type.to_string();
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
                            <button class:active=show_chart on:click=move |_| set_show_chart.set(true)>
                                "Analytics"
                            </button>
                            <button class:active=move || !show_chart.get() on:click=move |_| set_show_chart.set(false)>
                                "Raw Data"
                            </button>
                        </div>
                    </div>
                }.into_any()
            }
            None => view! { <p class="empty-state">"No robot selected"</p> }.into_any(),
        }}
    }
}