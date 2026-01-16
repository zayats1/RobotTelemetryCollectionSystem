use leptos::prelude::*;
use tabled::Tabled;
use serde::Serialize;
use csv::WriterBuilder;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, Url, Blob, BlobPropertyBag};

// 1. Your CSV Export Logic
pub fn export_to_csv<T: Serialize>(data: &[T]) -> Result<String, Box<dyn std::error::Error>> {
    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_writer(vec![]);

    for item in data {
        wtr.serialize(item)?;
    }

    let inner = wtr.into_inner()?;
    Ok(String::from_utf8(inner)?)
}

// 2. The Browser Download Trigger
fn trigger_download(csv_content: String, filename: &str) {
    let mut options = BlobPropertyBag::new();

    let _ = js_sys::Reflect::set(
        &options,
        &"type".into(),
        &"text/csv".into(),
    );
    let parts = js_sys::Array::of1(&csv_content.into());
    if let Ok(blob) = Blob::new_with_blob_sequence_and_options(&parts, &options) {
        if let Ok(url) = Url::create_object_url_with_blob(&blob) {
            let document = web_sys::window().unwrap().document().unwrap();
            let link: web_sys::HtmlAnchorElement = document.create_element("a")
                .unwrap()
                .unchecked_into();
            link.set_href(&url);
            link.set_download(filename);
            link.click();
            let _ = Url::revoke_object_url(&url);
        }
    }
}

// 3. The Component
#[component]
pub fn InfoTable<T>(
    #[prop(into)] data: Vec<T>
) -> impl IntoView
where
    T: Tabled + Serialize + Clone + 'static
{
    // Clone data specifically for the download closure
    let data_for_download = data.clone();
    let on_download = move |_| {
        if let Ok(csv_string) = export_to_csv(&data_for_download) {
            trigger_download(csv_string, "table_export.csv");
        }
    };

    // Clone data specifically for the display closure
    let data_for_display = data.clone();
    let table_display = move || {
        let mut table = tabled::Table::new(&data_for_display);
        table.with(tabled::settings::Style::psql());
        table.to_string()
    };

    view! {
        <div class="info_table_container">
            <div class="table_actions" style="margin-bottom: 0.5rem;">
                <button
                    on:click=on_download
                    class="download_button"
                >
                    "Download CSV"
                </button>
            </div>

            <div class="info_table" style="overflow-x: auto;">
                <pre style="font-family: monospace; white-space: pre;">
                    {table_display()}
                </pre>
            </div>
        </div>
    }
}