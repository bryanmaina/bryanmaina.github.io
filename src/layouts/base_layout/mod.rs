use leptos::prelude::*;
use leptos_router::components::{Outlet, A};
use thaw::*;

#[component]
pub fn BaseLayout() -> impl IntoView {
    view! {
        <Layout has_sider=true>
            <LayoutSider attr:style="background-color: #0078ff99; padding: 20px;">
                "Sider"
            </LayoutSider>
            <Layout>
                <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                    <nav class="flex space-x-4">
                        <A href="/">"Blog"</A>
                        <A href="/about">"About"</A>
                    </nav>
                </LayoutHeader>
                <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                    <Outlet />
                </Layout>
            </Layout>
        </Layout>
    }
}
