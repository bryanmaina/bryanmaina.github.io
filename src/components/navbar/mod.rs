use leptos::html::Header;
use leptos::prelude::*;
use leptos::web_sys::Event;
use leptos::web_sys::MouseEvent;
use leptos_router::components::A;
use leptos_use::use_document;
use leptos_use::{on_click_outside, use_resize_observer};
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::components::svg::ArrowRight;
use crate::components::svg::MLogo;

#[component]
pub fn NavBar(
    nav_show: Signal<bool>,
    nav_toggle: impl FnMut(MouseEvent) + 'static,
    nav_hide: impl FnMut(Event) + 'static + std::clone::Clone,
    nav_height: ReadSignal<f64>,
    nav_height_setter: WriteSignal<f64>,
) -> impl IntoView {
    let store = use_context::<Store<GlobalState>>()
        .expect("could not find global state store in the provided context");

    let main_nav_position = store.main_nav_position();

    let position_style = move || main_nav_position.get();

    let nav_el = NodeRef::<Header>::new();

    use_resize_observer(nav_el, move |entries, _| {
        let rect = entries[0].content_rect();
        nav_height_setter.set(rect.height());
    });

    let _ = on_click_outside(nav_el, nav_hide.clone());

    let menu_txt = move || match nav_show.get() {
        true => "Close",
        false => "Menu",
    };

    let document = use_document();

    Effect::new(move |_| {
        if nav_show.get() {
            document.body().unwrap().set_class_name("cannotscroll");
        } else {
            document.body().unwrap().set_class_name("canscroll");
        }
    });

    view! {
        <header
            class="z-50 grid w-full grid-cols-25 items-center rounded-b-3xl bg-bm-white/30 py-3 backdrop-blur-2xl [&:*]:h-full"
            style:position=position_style
            node_ref=nav_el
        >
            <div class="col-start-3 col-end-[-2] flex justify-between">
                <A href="" {..} class="inline-flex">
                    <MLogo {..} class="h-11" />
                </A>
                <button
                    type="button"
                    on:click=nav_toggle
                    class="font-bold lowercase"
                    aria-label=menu_txt
                >
                    {menu_txt}
                </button>
                <nav
                    class="absolute top-0 left-0 -z-10 h-fit w-full flex-col gap-4 bg-bm-white pb-4"
                    style:padding-top=move || {
                        format!("calc((0.75rem * 2) + {}px)", nav_height.get())
                    }
                    style:max-height=move || format!("calc({}px + 100vh)", nav_height.get())
                    style:display=move || if nav_show.get() { "flex" } else { "none" }
                    role="navigation"
                    aria-label="Menu"
                >
                    <ul
                        role="menu"
                        aria-label="main menu"
                        class="grid w-full list-none grid-cols-25 items-center gap-y-10 rounded-4xl bg-bm-accent pt-12 pb-12"
                    >
                        <NavItem navigate_to="" name="Blog" nav_hide=nav_hide.clone() />
                        <NavItem
                            navigate_to="/?q=projects"
                            name="Projects"
                            nav_hide=nav_hide.clone()
                        />
                        <NavItem navigate_to="/?q=about" name="About" nav_hide=nav_hide.clone() />
                    </ul>
                    <ContactButton />
                </nav>
            </div>
        </header>
    }
}

#[component]
fn ContactButton() -> impl IntoView {
    view! {
        <button class="relative inline-flex w-full justify-between px-[calc((2_/_25)_*_100%)] py-8 before:absolute before:top-0 before:left-0 before:-z-10 before:h-full before:w-full before:rounded-4xl before:bg-bm-silver">
            <span class="cursor-pointer text-2xl leading-snug font-medium">"Contact"</span>
            <span class="flex w-12 rounded-md bg-black/10 px-3.5 py-2.5">
                <ArrowRight />
            </span>
        </button>
    }
}

#[component]
fn NavItem(
    #[prop(into)] navigate_to: String,
    #[prop(into)] name: String,
    mut nav_hide: impl FnMut(Event) + 'static + std::clone::Clone,
) -> impl IntoView {
    view! {
        <li role="menuitem" class="col-start-3 col-end-[-3]">
            <A
                href=navigate_to.clone()
                on:click=move |ev| { nav_hide(ev.into()) }
                exact=true
                {..}
                class="item-center box-border flex w-full p-0 text-left"
            >
                <span class="cursor-pointer text-2xl leading-snug font-medium">{name}</span>
            </A>
        </li>
    }
}
