use leptos::prelude::*;
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    components::{blog_sub_navbar::BlogSubNavbar, count_btn::Button},
};

#[component]
pub fn HomePage(
    #[prop(optional, default = "relative".to_string())] main_navbar_position: String,
) -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();
    let main_nav_position = store.main_nav_position();
    *main_nav_position.write() = main_navbar_position.to_string();

    view! {
        <BlogSubNavbar />
        <main>
            <div class="bg-red-500 h-screen">
                <h1>{move || main_nav_position.get()}</h1>
                <p>"List of articles:"</p>
                <ul>
                    <li>
                        <a href="/?q=blog&article=how-to-communicate">"How to Communicate"</a>
                    </li>
                    <li>
                        <a href="/?q=blog&article=rust-basics">"Rust Basics"</a>
                    </li>
                </ul>
            </div>
            <Button />
            <pre>{move || serde_json::to_string_pretty(&*store.read())}</pre>
        </main>
    }
}
