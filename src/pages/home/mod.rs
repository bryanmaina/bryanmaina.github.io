use leptos::prelude::*;
use leptos_meta::*;

use crate::components::count_btn::Button;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <Title text="blog" />
            <h1 class="text-3xl font-bold text-green-800 underline">"Welcome to Leptos!"</h1>
            <h2 class="text-2xl text-red-400">Tailwind is working as expected</h2>
            <Button />
        </main>
    }
}


