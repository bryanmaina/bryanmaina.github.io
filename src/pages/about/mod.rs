use leptos::prelude::*;
use leptos_meta::*;

use crate::components::navbar::set_main_navbar_position;

#[component]
pub fn AboutPage(
    #[prop(optional, default = "fixed".to_string())] main_navbar_position: String,
) -> impl IntoView {
    set_main_navbar_position(main_navbar_position);
    view! {
        <main>
            <Title text="about" />
            <h1 class="text-3xl">"About"</h1>
        </main>
    }
}
