use leptos::prelude::*;

use crate::components::{
    blog_sub_navbar::BlogSubNavbar, count_btn::Button, navbar::set_main_navbar_position,
};

#[component]
pub fn HomePage(
    #[prop(optional, default = "relative".to_string())] main_navbar_position: String,
) -> impl IntoView {
    set_main_navbar_position(main_navbar_position);

    view! {
        <BlogSubNavbar />
        <main>
            <div class="bg-red-500 h-screen">
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
        </main>
    }
}
