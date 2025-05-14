use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn Button() -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();
    let count = store.count();
    let main_nav_position = store.main_nav_position();
    let on_click = move |_| *count.write() += 1;

    view! {
        <button on:click=on_click>"Click Me: " {move || count.get()}</button>
        <span>"Count is: " {move || count.get()}</span>
        <span>"Count is: " {move || main_nav_position.get()}</span>
    }
}
