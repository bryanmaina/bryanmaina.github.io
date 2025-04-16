use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::use_query_map,
    StaticSegment,
};

use crate::{
    layouts::base_layout::BaseLayout,
    pages::{about::AboutPage, home::HomePage},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let formatter = |text| format!("{text} — Bryan Maina");

    view! {
        <Title formatter />
        <Link rel="shortcut icon" type_="image/ico" href="/public/favicon.ico" />

        <Router>
            <Routes fallback=|| "Page not found." transition=true>
                <ParentRoute path=StaticSegment("") view=BaseLayout>
                    <Route path=StaticSegment("") view=ViewSelector />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn ViewSelector() -> impl IntoView {
    let query_params = use_query_map();
    let current_view = move || match query_params.read().get("q").as_deref() {
        Some("blog") => match query_params.read().get("article").as_deref() {
            Some(article_slug) => BlogPostPage(BlogPostPageProps {
                slug: article_slug.to_string(),
            })
            .into_any(),
            None => BlogListPage().into_any(),
        },
        Some("about") => AboutPage().into_any(),
        None => HomePage().into_any(),
        Some(_) => HomePage().into_any(),
    };
    current_view
}

#[component]
fn BlogPostPage(slug: String) -> impl IntoView {
    view! {
        <h1>"Blog Post"</h1>
        // Here you would fetch and display the post based on the slug
        <p>"Displaying article: " {slug}</p>
    }
}

#[component]
fn BlogListPage() -> impl IntoView {
    view! {
        <h1>"Blog"</h1>
        <p>"List of articles:"</p>
        <ul>
            // Example link to a specific article
            <li>
                <a href="/?q=blog&article=how-to-communicate">"How to Communicate"</a>
            </li>
            <li>
                <a href="/?q=blog&article=rust-basics">"Rust Basics"</a>
            </li>
        </ul>
    }
}
