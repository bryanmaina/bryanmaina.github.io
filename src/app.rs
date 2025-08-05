use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{ParentRoute, Route, Router, Routes},
    hooks::use_query_map,
};
use reactive_stores::Store;
use serde::Serialize;

use crate::{
    components::blog_article::ArticleLoader,
    layouts::base_layout::BaseLayout,
    pages::{about::AboutPage, home::HomePage, projects::ProjectsPage},
};

#[derive(Debug, Clone, Default, Store, Serialize)]
pub struct GlobalState {
    count: i32,
    main_nav_position: String,
    main_nav_expanded: bool,
    sub_nav_expanded: bool,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let store = Store::new(GlobalState::default());
    provide_context(store);

    let formatter = |text| format!("{text} — Bryan Maina");

    view! {
        <Title formatter />
        <Link rel="shortcut icon" type_="image/ico" href="/public/favicon.ico" />

        <Meta
            name="google-site-verification"
            content="qlX_0XoEtjchKYQtMxHCwGMF0YFyIuWP-YtojhAi8uk"
        />

        <Meta property="theme-color" content="#002b77" />

        <Meta property="og:site_name" content="Bmaina logs" />

        <Meta property="twitter:domain" content="bryanmaina.github.io" />

        <MetaTags />
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

    move || match query_params.read().get("q").as_deref() {
        Some("blog") => match query_params.read().get("article").as_deref() {
            Some(article_slug) => BlogPostPage(BlogPostPageProps {
                slug: article_slug.to_string(),
            })
            .into_any(),
            None => view! { <HomePage /> }.into_any(),
        },
        Some("about") => view! { <AboutPage /> }.into_any(),
        Some("projects") => view! { <ProjectsPage /> }.into_any(),
        None => view! { <HomePage /> }.into_any(),
        Some(_) => view! { <HomePage /> }.into_any(),
    }
}

#[component]
fn BlogPostPage(slug: String) -> impl IntoView {
    view! {
        <h1>"Blog Post"</h1>
        // Here you would fetch and display the post based on the slug
        // <p>"Displaying article: " {slug}</p>
        <ArticleLoader slug=slug />
    }
}
