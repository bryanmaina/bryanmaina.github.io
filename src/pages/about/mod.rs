use leptos::prelude::*;
use leptos_meta::*;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn AboutPage(
    #[prop(optional, default = "fixed".to_string())] main_navbar_position: String,
) -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();
    let main_nav_position = store.main_nav_position();
    *main_nav_position.write() = main_navbar_position.to_string();

    view! {
        <main>
            <Title text="about" />
            <h1 class="text-3xl">"About"</h1>
        </main>
    }
}
