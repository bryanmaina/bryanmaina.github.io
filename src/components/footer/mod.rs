use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative mt-12 w-full">
            <div class="container mx-auto px-12 py-12">
                <div class="grid grid-cols-1 gap-12 md:grid-cols-3">
                    <FooterAboutTheBlog />
                    <FooterNavigation />
                    <FooterBuiltWith />
                </div>
                <FooterSocials />
            </div>
        </footer>
    }
}

#[component]
fn FooterAboutTheBlog() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h2 class="font-bold">"About this Blog!"</h2>
            <p class="text-sm">
                "Expect insightful artcles, pratical guides, maybe a peek into the AI crystal ball, and thought-provoking explorations. Hopefully some genuinely usefull stuff. We learn together, we fail together!"
            </p>
        </div>
    }
}

#[component]
fn FooterNavigation() -> impl IntoView {
    const LINKS: &[(&str, &str)] = &[
        ("Home", "/"),
        ("Projects", "/?q=projects"),
        ("About", "/?q=about"),
    ];

    view! {
        <div>
            <h3 class="mb-6 text-sm font-medium tracking-wide uppercase">"Navigation"</h3>
            <ul class="space-y-3">
                {LINKS
                    .iter()
                    .map(|(name, href)| {
                        view! {
                            <li>
                                <a href=*href class="group inline-block">
                                    <div class="translate-x-0 transform transition-all duration-300 ease-in-out group-hover:translate-x-1 group-hover:text-bm-blue">
                                        {*name}
                                    </div>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn FooterBuiltWith() -> impl IntoView {
    const TECHS: &[(&str, &str)] = &[
        ("Rust", "https://www.rust-lang.org/"),
        ("WebAssembly", "https://webassembly.org/"),
        ("Leptos", "https://leptos.dev/"),
        ("Tailwind", "https://tailwindcss.com/"),
        ("Leptos-Use", "https://leptos-use.rs/"),
    ];

    view! {
        <div>
            <h3 class="mb-6 text-sm font-medium tracking-wide uppercase">"Built with"</h3>
            <div class="grid grid-cols-3 gap-3">
                {TECHS
                    .iter()
                    .map(|(name, href)| {
                        view! {
                            <a href=*href target="_blank" rel="external noopener noreferrer">
                                <div class="rounded-lg border border-bm-light-silver px-3 py-1.5 text-xs">
                                    {*name}
                                </div>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn FooterSocials() -> impl IntoView {
    const SOCIALS: &[(&str, &str, &str)] = &[
        (
            "GitHub",
            "https://github.com/bryanmaina/",
            "https://cdn.custom-cursor.com/db/pointer/32/Spongebob_Imagination_Pointer.png",
        ),
        (
            "LinkedIn",
            "https://www.linkedin.com/in/bryan-maina/",
            "https://cdn.custom-cursor.com/db/cursor/32/gta_5_michael_and_money_cursor.png",
        ),
        (
            "Twitter",
            "https://x.com/binarymaan",
            "https://cdn.custom-cursor.com/db/cursor/32/star_wars_old_luke_skywalker_green_lightsaber_cursor.png",
        ),
        (
            "Dev",
            "https://dev.to/bryanmaina",
            "https://cdn.custom-cursor.com/db/7811/32/spongebob-old-man-walker-pointer.png",
        ),
    ];

    view! {
        <div class="flex space-x-4 pt-12 md:pt-0">
            {SOCIALS
                .iter()
                .map(|(name, href, cursor)| {
                    view! {
                        <a
                            href=*href
                            target="_blank"
                            rel="external noopener noreferrer"
                            class="group inline-block"
                            style:cursor=move || { format!("url({}), pointer", *cursor) }
                        >
                            <div
                                data-after="💛"
                                class="after:content-['.'] font-medium text-lg translate-x-0 transform transition-all duration-300 ease-in-out group-hover:-translate-2 group-hover:text-bm-blue"
                            >
                                {*name}
                            </div>
                        </a>
                    }
                })
                .collect_view()}

        </div>
    }
}
