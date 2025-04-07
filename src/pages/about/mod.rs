use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <main>
            <Title text="about" />
            <h1 class="text-3xl">"About"</h1>
        </main>
    }
}
