use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    components::svg::ArrowDown,
};
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn BlogSubNavbar() -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();

    const TOPICS: &[(&str, &str)] = &[
        ("Java", "/?q=blog&topic=java"),
        ("Rust", "/?q=blog&topic=rust"),
        ("AI", "/?q=blog&topic=ai"),
        ("I hate AI", "/?q=blog&topic=i-hate-ai"),
    ];

    let sub_nav_expanded_field = store.sub_nav_expanded();
    let display_style_value = move || {
        if sub_nav_expanded_field.get() {
            "block"
        } else {
            "none"
        }
    };

    view! {
        <div class="sticky top-0 left-0 grid grid-cols-25 bg-bm-white">
            <nav class="relative col-start-2 col-end-[-2]" aria-label="Blog main navigation">
                <BlogNavButton />
                <ul
                    id="blog-topics-dropdown"
                    class="absolute z-10 mt-2 w-56 rounded-md border border-gray-200 bg-white py-1 shadow-lg"
                    style:display=display_style_value
                >
                    {TOPICS
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <li>
                                    <a
                                        href=*href
                                        class="block px-4 py-2 text-sm leading-[1.4] font-medium text-gray-700 hover:bg-gray-100 hover:text-gray-900"
                                    >
                                        {*name}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
        </div>
    }
}

#[component]
fn BlogNavButton() -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();

    let sub_nav_expanded = store.sub_nav_expanded();
    view! {
        <button
            type="button"
            class="inline-flex max-w-full items-center gap-4 px-5 py-3.5"
            on:click=move |_| { *sub_nav_expanded.write() = !sub_nav_expanded.get() }
            aria-expanded=move || sub_nav_expanded.get().to_string()
            aria-controls="blog-topics-dropdown"
        >
            <span class=" text-lg font-bold text-nowrap">"Blog"</span>
            <ArrowDown {..} class="w-6" />
        </button>
    }
}
