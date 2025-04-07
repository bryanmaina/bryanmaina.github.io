use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{ParentRoute, Route, Router, Routes}, StaticSegment};

use crate::{layouts::base_layout::BaseLayout, pages::{about::AboutPage,  home::HomePage}};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
        let formatter = |text| format!("{text} — Bryan Maina");

    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Title formatter />
        <Link rel="shortcut icon" type_="image/ico" href="/public/favicon.ico" />

        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=StaticSegment("") view=BaseLayout>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("about") view=AboutPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
