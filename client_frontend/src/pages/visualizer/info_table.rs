use leptos::prelude::*;
use tabled::Tabled;
use robot_data::robot_info::BatteryInfo;

#[component]
pub fn InfoTable<T: Tabled>(
    #[prop(into)]
    data: Vec<T>
) -> impl IntoView {
    let table_display = move || {
        let mut table = tabled::Table::new(&data);
        table.with(tabled::settings::Style::psql());
        table.to_string()
    };

    view! {
        <div class="info_table">
            <pre>{table_display()}</pre>
        </div>
    }
}