mod author_info;

use author_info::*;

use gloo_net::http::Request;
use gray_matter::{Matter, engine::YAML};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_meta::{Meta, Script, Title};
use leptos_router::components::A;
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag};
use serde::Deserialize;
use slug::slugify;

const WEBSITE_BASE: &str = "https://bryanmaina.github.io";
const GITHUB_BLOG_RAW_BASE: &str =
    "https://raw.githubusercontent.com/bryanmaina/bryanmaina.github.io/main/content/blog";
const LANDSCAPE: &str = "16x9";
const SQUARE: &str = "1x1";
const PORTRAIT: &str = "4x3";
const IMAGE_RATIOS: [&str; 3] = [LANDSCAPE, SQUARE, PORTRAIT];

#[derive(Debug, Clone)]
pub struct Article {
    pub article_metadata: ArticleMetadata,
    pub article_content: String,
    pub slug: String,
    pub toc_items: Vec<TocItem>,
}

#[derive(Debug, Clone)]
pub struct TocItem {
    pub title: String,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArticleMetadata {
    pub github_link: Option<String>,
    pub demo_link: Option<String>,
    pub categories: Vec<String>,
    pub collections: Vec<String>,
    pub seo: SeoMetadata,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SeoMetadata {
    pub title: String,
    pub description: String,

    #[serde(rename = "author:name")]
    pub author_name: String,

    #[serde(rename = "author:github")]
    pub author_github: String,

    #[serde(rename = "author:linkedin")]
    pub author_linkedin: String,

    #[serde(rename = "author:twitter")]
    pub author_twitter: String,

    #[serde(rename = "twitter:card")]
    pub twitter_card: String,

    pub keywords: Vec<String>,

    #[serde(rename = "og:locale")]
    pub og_locale: String,

    #[serde(rename = "og:type")]
    pub og_type: String,

    #[serde(rename = "og:image:alt")]
    pub og_image_alt: String,

    #[serde(rename = "article:section")]
    pub section: String,

    pub modified_time: String,
    pub published_time: String,
    pub time_to_read: String,
}

impl Article {
    pub fn new<S: AsRef<str>>(
        content: S,
        article_metadata: ArticleMetadata,
        slug: S,
        toc_items: Vec<TocItem>,
    ) -> Self {
        Self {
            article_metadata,
            article_content: content.as_ref().to_owned(),
            slug: slug.as_ref().to_owned(),
            toc_items,
        }
    }
}

fn parse_content<S: AsRef<str>>(content: S, slug: String) -> Option<Article> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let matter = Matter::<YAML>::new();

    match matter.parse::<ArticleMetadata>(content.as_ref()) {
        Ok(parsed_article) => parsed_article.data.map(|article_metadata| {
            let article_content = parsed_article.content;
            let mut toc_items = Vec::new();
            let mut previous_id: Option<CowStr<'_>> = None;
            let parser = Parser::new_ext(&article_content, options).map(|event| match event {
                Event::Start(Tag::Image {
                    id,
                    link_type,
                    dest_url,
                    title,
                }) => {
                    let new_url = if !dest_url.contains("://") {
                        // Only transform relative URLs
                        format!("{}/{}/{}", GITHUB_BLOG_RAW_BASE, slug, dest_url)
                    } else {
                        dest_url.to_string()
                    };
                    Event::Start(Tag::Image {
                        id,
                        link_type,
                        dest_url: new_url.into(),
                        title,
                    })
                }
                // Event::Start(Tag::Heading {
                //     level: HeadingLevel::H2,
                //     ref id,
                //     classes: _,
                //     attrs: _,
                // }) => {
                //     if let Some(idp) = id {
                //         console_warn("we are here");
                //         toc_items.push(TocItem {
                //             title: "home".to_string(),
                //             id: idp.to_string(),
                //         });
                //     }
                //     event
                // }
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H2,
                    ref id,
                    classes: _,
                    attrs: _,
                }) => {
                    previous_id = id.clone();
                    event
                }
                Event::Text(ref text) => {
                    let Some(id) = previous_id.take() else {
                        return event;
                    };
                    toc_items.push(TocItem {
                        title: text.to_string(),
                        id: id.to_string(),
                    });
                    previous_id = None;
                    event
                }
                e => e,
            });
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Article {
                article_metadata,
                article_content: html_output,
                slug,
                toc_items,
            }
        }),
        Err(e) => {
            log::error!("Failed to parse content: {e:?}");
            None
        }
    }
}

async fn retrieve_article(slug: String) -> Option<(String, String)> {
    let url = format!("{GITHUB_BLOG_RAW_BASE}/{slug}/content.md");
    let req = Request::get(url.as_str());
    match req.send().await {
        Ok(res) => (res.text().await).ok().map(|content| (content, slug)),
        Err(_) => None,
    }
}

#[component]
pub fn ArticleLoader(#[prop(into)] slug: String) -> impl IntoView {
    let async_content = LocalResource::new(move || retrieve_article(slug.clone()));

    let parsed_content = move |content, slug| match parse_content(content, slug) {
        Some(article) => view! {
            <MarkdownViewer article=article />
            <script src="/public/highlighter/load_highlight.js" defer></script>
        }
        .into_any(),
        None => {
            view! { <div class="p-4 text-red-500">"Failed to parse the content"</div> }.into_any()
        }
    };

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="animate-pulse p-4">
                    <div class="h-4 w-3/4 rounded bg-gray-200"></div>
                    <div class="mt-4 space-y-3">
                        <div class="h-4 rounded bg-gray-200"></div>
                        <div class="h-4 rounded bg-gray-200"></div>
                    </div>
                </div>
            }
        }>
            {move || match async_content.get() {
                Some(Some((content, slug))) => parsed_content(content, slug),
                Some(None) => {

                    view! { <div class="p-4 text-red-500">"Failed to load article content"</div> }
                        .into_any()
                }
                None => view! { <div>"Loading..."</div> }.into_any(),
            }}
        </Suspense>
    }
}

#[component]
pub fn MarkdownViewer(#[prop(into)] article: Article) -> impl IntoView {
    let cannonical_ulr = format!("{}/?q=blog&article={}", WEBSITE_BASE, article.slug);

    let image_with_ratio = |i| format!("{GITHUB_BLOG_RAW_BASE}/{}/splash-{i}.png", article.slug);
    // let image_urls = format!("{GITHUB_BLOG_RAW_BASE}/{}/", article.slug);
    let image_urls: Vec<String> = IMAGE_RATIOS
        .iter()
        .map(image_with_ratio)
        .collect::<Vec<String>>();

    let seo_metadata = article.article_metadata.seo;

    view! {
        <Title text=seo_metadata.title.clone() />
        <Meta name="description" content=seo_metadata.description.clone() />
        <Meta name="keywords" content=seo_metadata.keywords.join(", ") />
        <Meta name="author" content=seo_metadata.author_name.clone() />

        // Open Graph tags
        <Meta property="og:url" content=cannonical_ulr.clone() />
        <Meta property="og:type" content=seo_metadata.og_type />
        <Meta property="og:title" content=seo_metadata.title.clone() />
        <Meta property="og:description" content=seo_metadata.description.clone() />
        <Meta property="og:image" content=image_with_ratio(&LANDSCAPE) />
        <Meta property="og:local" content=seo_metadata.og_locale />

        // Twitter Card Tag
        <Meta name="twitter:card" content=seo_metadata.twitter_card />
        <Meta property="twitter:url" content=cannonical_ulr />
        <Meta name="twitter:site" content=seo_metadata.author_twitter.clone() />
        <Meta name="twitter:title" content=seo_metadata.title.clone() />
        <Meta name="twitter:image" content=image_with_ratio(&LANDSCAPE) />
        <Meta name="twitter:creator" content=seo_metadata.author_twitter />
        <Meta name="twitter:description" content=seo_metadata.description.clone() />

        // Article Specific Meta Tags
        <Meta property="article:author" content=seo_metadata.author_name.clone() />
        <Meta property="article:publisher" content=WEBSITE_BASE />
        <Meta property="article:published_time" content=seo_metadata.published_time.clone() />
        <Meta property="article:modified_time" content=seo_metadata.modified_time.clone() />
        <Meta property="article:section" content=seo_metadata.section.clone() />
        <Meta name="reading-time" content=seo_metadata.time_to_read.clone() />

        <Script type_="application/ld+json">
            {format!(
                r#"{{
                    "@context": "https://schema.org",
                    "@type": "Article",
                    "headline": "{}",
                    "description": "{}",
                    "author": {{
                        "@type": "Person",
                        "name": "{}",
                        "url": "{}"
                    }},
                    "dateModified": "{}",
                    "image": [
                        {}
                    ],
                    "publisher": {{
                        "@type": "Organization",
                        "name": "Bryan Maina's Blog",
                        "url": "{}",
                        "logo": {{
                            "@type": "ImageObject",
                            "url": "{}/logo.png"
                        }}
                    }}
                }}"#,
                seo_metadata.title.clone(),
                seo_metadata.description.clone(),
                seo_metadata.author_name.clone(),
                WEBSITE_BASE,
                seo_metadata.modified_time.clone(),
                image_urls.iter().map(|url| format!(r#""{url}""#)).collect::<Vec<_>>().join(","),
                WEBSITE_BASE,
                WEBSITE_BASE,
            )}
        </Script>

        <div class="grid grid-cols-25">
            <div class="col-start-2 col-end-[-2] flex flex-col gap-8">

                <ArticleInfo
                    section=seo_metadata.section
                    title=seo_metadata.title
                    time_to_read=seo_metadata.time_to_read
                    modified_time=seo_metadata.modified_time
                />

                <img
                    src=image_with_ratio(&LANDSCAPE)
                    alt="article splash screen"
                    class="aspect-345/149 rounded-3xl object-cover"
                />

                <div class="flex py-12">
                    <ArticleSidebar
                        github_username=seo_metadata.author_github
                        toc_items=article.toc_items
                    />

                    <div
                        class="markdown-content prose max-w-none [&_h2]:text-2xl [&_h2]:font-bold [&_p]:text-lg [&*>]:font-inter"
                        inner_html=article.article_content
                    />
                </div>
            </div>

        </div>
    }
}

#[component]
pub fn ArticleSidebar(
    #[prop(into)] github_username: String,
    #[prop(into)] toc_items: Vec<TocItem>,
) -> impl IntoView {
    view! {
        <div class="md:min-h-dvh md:w-[25rem] md:px-6">
            <nav class="mb-8">
                <h3 class="text-lg font-semibold mb-4">"Table of Contents"</h3>
                <ul class="space-y-2">
                    {toc_items
                        .into_iter()
                        .map(|item| {
                            console_log(&item.title);
                            view! {
                                <li>
                                    <a
                                        href=format!("#{}", item.id)
                                        class="text-gray-600 hover:text-gray-900 block py-1"
                                    >
                                        {item.title}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
            <AuthorInfo github_username />
        </div>
    }
}

#[component]
pub fn ArticleInfo(
    #[prop(into)] section: String,
    #[prop(into)] title: String,
    #[prop(into)] time_to_read: String,
    #[prop(into)] modified_time: String,
) -> impl IntoView {
    view! {
        <div class="flex w-full flex-col items-center gap-3.5 px-21.5 pt-6.5 font-inter">
            <ArticleTag text=section />
            <h1 class="max-w-1/2 text-center text-4xl font-medium text-balance">{title}</h1>
            <p class="text-lg leading-[1.15]">
                {time_to_read} " min read · " <span title="Last Updated">{modified_time}</span>
            </p>
        </div>
    }
}

#[component]
pub fn ArticleTag(#[prop(into)] text: String) -> impl IntoView {
    let target = slugify(&text);

    view! {
        <A
            href=target
            attr:class="rounded-full bg-[#f6f6f6] px-4.5 py-2.5 font-inter text-lg leading-[1.2] font-semibold tracking-[-0.6] text-[#4d4d4d]"
        >
            {text}
        </A>
    }
}
