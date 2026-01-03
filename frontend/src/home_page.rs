use leptos::{component, view, IntoView};
use leptos::prelude::{GlobalAttributes, OnAttribute, RwSignal, Write};
use leptos_meta::Stylesheet;
use leptos::prelude::*;
/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Stylesheet id="leptos" href="/home_page.css"/>
        <button on:click={on_click}>"Click Me: " {move || count.get()}</button>

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