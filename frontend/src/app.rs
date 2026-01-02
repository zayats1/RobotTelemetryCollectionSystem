use crate::visualizer::Visualizer;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::robot_map::RobotMap;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/telemetry-collector.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <nav>
           <p>"Welcome to Leptos!"</p>
          <a href="/Visualizer/">Visualizer</a>
          <a href="/HomePage/">HomePage</a>
          <a href="/RobotMap/">RobotMap</a>
        </nav>
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("HomePage") view=HomePage/>
                    <Route path=StaticSegment("Visualizer") view=Visualizer/>
                    <Route path=StaticSegment("RobotMap") view=RobotMap/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Stylesheet id="leptos" href="/home_page.css"/>
        <button on:click=on_click>"Click Me: " {count}</button>

        <section id="gallery">
        <article>
            <img class="image" src="pictures/AndromedaGalaxy.jpg" alt="se la vi"/>
            <p>
                <b>"Галактика Андромеди(інша назва: Туманність Андромеди) й галактика M110"</b>
               "ISO 3200
                Витримка 5 -8(точно не пам'ятаю) секунд
                камера Canon 1100Da(модифікована для астрономії)
                об'єктив подарованний f 2.0 F 92
                Знято з нерухомого штатива
                Одиночний кадр
                оброблено в Darktable"
            </p>
        </article>
        <article>
            <img class="image" src="pictures/METEOR.jpg" alt="se la vi" />
            <p>
                <b>
                "Великий болід у сузір'ях Кассіопея та Цефей"
                    </b>
               "exp 30sec
                f 3,5
                sigma 18/200 DC II
                Canon 1100Da (Baader IR filter)
                пост обр. Adobe Photoshop CC 2020 Camera Raw"
            </p>
        </article>
        <article>
            <img class="image" src="pictures/neowise.jpg" alt="se la vi"/>
            <p> <b>"С/2020 f3 NEOWISE у сузірї Рись 15.07.2020"</b>
                "Витримка 10 секунд
                ISO 3200
                Діафрагма f 6,3
                Фокусна відстань- 200мм
                Canon1100Da(modified with Baader IR filter)
                Sigma 18-200 DC II f3,5-6,3"
            </p>
        </article>
        <article>
            <img class="image" src="pictures/RiceOfMoon.jpg" alt="se la vi"/>
            <p> <b>"Місяць над селом"</b>
                "sigma 18/200 DC II Canon 1100Da (Baader IR filter) пост обр. Adobe Photoshop CC 2020 Camera Raw"
            </p>
        </article>
        <article>
            <img class="image" src="pictures/VenusInPleiades.jpg" alt="se la vi"/>
            <p>
                <b>"Оце мені так пощастило!Венера в Плеядах."</b>
                "Знято фото було 4 квітня 2020 року в 22-23 годині."
            </p>
        </article>
    </section>
    }
}
