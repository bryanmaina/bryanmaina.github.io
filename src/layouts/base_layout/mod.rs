use leptos::prelude::*;
use leptos_router::components::{Outlet, A};
use thaw::*;

#[component]
pub fn BaseLayout() -> impl IntoView {
    view! {
        <Layout>
            <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                <nav class="flex space-x-4">
                    <A href="">"Home"</A>
                    <A href="/?q=blog">"Blog"</A>
                    <A href="/?q=about">"About"</A>
                </nav>
            </LayoutHeader>
            <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                <Outlet />
            </Layout>
        </Layout>
    }
}
