use leptos::prelude::*;
use thaw::{Button as ThawButton, *};

#[component]
pub fn Button() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <ConfigProvider>
            <ThawButton on:click=on_click appearance=ButtonAppearance::Primary>
                "Click Me: "
                {count}
            </ThawButton>
        </ConfigProvider>
    }
}
