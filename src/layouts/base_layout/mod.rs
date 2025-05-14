use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_use::{use_toggle, UseToggleReturn};

use crate::components::navbar::NavBar;

#[component]
pub fn BaseLayout() -> impl IntoView {
    let (nav_height, nav_height_setter) = signal(0_f64);

    let UseToggleReturn {
        toggle: toggle_navbar,
        value: show_navbar,
        set_value: set_navbar_value,
    } = use_toggle(false);

    view! {
        <div class="flex flex-col items-stretch">
            <div
                class="w-full"
                style:height=move || format!("calc((0.75rem * 2) + {}px)", nav_height.get())
            >
                <NavBar
                    nav_show=show_navbar
                    nav_toggle=move |_| toggle_navbar()
                    nav_hide=move |_| set_navbar_value(false)
                    nav_height
                    nav_height_setter
                />
            </div>
            <div class="h-[300vh]">
                <div
                    class="fixed top-0 z-40 h-screen w-screen cursor-pointer backdrop-blur-2xl"
                    style:display=move || { if show_navbar.get() { "block" } else { "none" } }
                ></div>
                <Outlet />
                <p class="text-3xl">{move || if show_navbar.get() { "true" } else { "false" }}</p>
            </div>
        </div>
    }
}
