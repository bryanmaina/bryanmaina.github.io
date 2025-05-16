use leptos::prelude::*;

use crate::components::navbar::set_main_navbar_position;

#[component]
pub fn ProjectsPage(
    #[prop(optional, default = "fixed".to_string())] main_navbar_position: String,
) -> impl IntoView {
    set_main_navbar_position(main_navbar_position);
    view! { <p>"My projects"</p> }
}
