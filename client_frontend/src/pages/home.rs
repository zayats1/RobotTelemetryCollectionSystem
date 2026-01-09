use crate::app_model::{AppModel, fetch_robots};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::prelude::{GlobalAttributes, OnAttribute, RwSignal, Write};
use leptos::svg::view;
use leptos::tachys::view;
use leptos::{IntoView, component, view};
use leptos_meta::Stylesheet;
use robot_data::robot_info::BasicInfo;
use std::sync::Arc;

use crate::pages::test_gallery::TestGallery;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {


    let model = use_context::<RwSignal<AppModel>>().expect("no model?");
    let robots = LocalResource::new(|| fetch_robots());

    view! {
        <section id="gallery">
            <Suspense fallback=|| {
                view! { <p>"Loading..."</p> }
            }>
                {move || Suspend::new(async move {
                    match robots.await {
                        Ok(data) => {
                            Either::Left({
                                let data: Vec<BasicInfo> = data.clone();
                                view! {
                                    <For
                                        each=move || data.clone()
                                        key=|x| x.id.clone()
                                        children=move |x| {
                                            view! {
                                                <article>
                                                    <img class="image" src="/pictures/DM_300_infobox.webp" alt="se la vi" />
                                                    <p> <b>"id: "</b>
                                                        {x.id}
                                                    </p>
                                                    <p> <b>"type: "</b>
                                            {x.robot_type.to_string()}</p>
                                                </article>
                                            }
                                        }
                                    />
                                }
                            })
                        }
                        Err(e) => {
                            Either::Right(
                                view! {
                                    <article>
                                        <p>{format!("Error: {e}")}</p>
                                    </article>
                                },
                            )
                        }
                    }
                })}

            </Suspense>
        // <TestGallery/> // for test
        </section>
    }
}
