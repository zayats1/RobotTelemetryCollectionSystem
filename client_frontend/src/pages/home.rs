use crate::fetcher::{ fetch_robots};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::prelude::{GlobalAttributes, OnAttribute, RwSignal, Write};
use leptos::{IntoView, component, view};
use robot_data::robot_info::BasicInfo;
use log::debug;
use crate::state::AppState;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let state = use_context::<RwSignal<AppState>>().expect("no state?");
    let robots = LocalResource::new(|| fetch_robots());
    let basic_info:RwSignal<Vec<BasicInfo>> = RwSignal::new(vec![]);


    let select_info = move |val:BasicInfo| {
        debug!("selected robot:{:?}", &val);
        let id = val.id.clone();
       let prev_state = state.get().clone();
       *state.write() = AppState {
           selected_info:Some(val.clone()),
           selected_id:Some(id.clone()),
           ..prev_state
       }

};
    view! {
        <section id="gallery">
            <Suspense fallback=|| {
                view! { <p>"Loading..."</p> }
            }>
                {move || Suspend::new(async move {
                    match robots.await {
                        Ok(data) => {
                            Either::Left({
                                *basic_info.write() = data;
                                view! {
                                    <For
                                        each=move || basic_info.get()
                                        key=|x| x.id.clone()

                                        children=move |x| {
                                            let val = x.clone();
                                            let the_id = val.id.clone();
                                            view! {
                                                <article
                                                    id=val.id.clone()
                                                    class:selected=move || {
                                                        state.get().selected_id == Some(the_id.clone())
                                                    }
                                                    on:click=move |_| select_info(val.clone())
                                                >
                                                    <img
                                                        class="image"
                                                        src="/pictures/DM_300_infobox.webp"
                                                        alt="se la vi"
                                                    />

                                                    <p>
                                                        <b>"id: "</b>
                                                        { x.id}
                                                    </p>
                                                    <p>
                                                        <b>"type: "</b>
                                                        {x.robot_type.to_string()}
                                                    </p>
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
        // <MockGallery/> // for test
        </section>
    }
}
