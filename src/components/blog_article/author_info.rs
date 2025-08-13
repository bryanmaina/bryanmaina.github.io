use gloo_net::http::Request;
use leptos::prelude::*;

use serde::Deserialize;

use crate::components::svg::{GitHubIcon, TwitterIcon};

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubUser {
    pub avatar_url: String,
    pub html_url: String,
    pub name: Option<String>,
    pub twitter_username: Option<String>,
    pub bio: Option<String>,
}

impl GitHubUser {
    fn split_bio(&self) -> (String, String) {
        match &self.bio {
            Some(bio) if bio.contains("--") => {
                let parts: Vec<&str> = bio.split("--").map(str::trim).collect();
                (parts[0].to_string(), parts[1].to_string())
            }
            Some(bio) => ("".to_string(), bio.to_string()),
            None => ("".to_string(), "".to_string()),
        }
    }
}

async fn retrieve_github_user_data(username: String) -> Option<GitHubUser> {
    let url = format!(
        "https://api.github.com/users/{}",
        username.trim_start_matches("@")
    );
    Request::get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-Github-Api-Version", "2022-11-28")
        .send()
        .await
        .ok()?
        .json::<GitHubUser>()
        .await
        .ok()
}

#[component]
pub fn AuthorInfo(#[prop(into)] github_username: String) -> impl IntoView {
    let user_data = LocalResource::new(move || retrieve_github_user_data(github_username.clone()));

    view! {
        <div class="flex h-fit flex-col gap-2.5 overflow-hidden pb-10 font-inter">
            <h3 class="text-sm leading-[1.2] font-semibold text-[#5f6368] uppercase">
                "About the author"
            </h3>

            <Suspense fallback=move || {
                view! {
                    <div class="animate-pulse p-4">
                        <div class="mb-4 h-24 w-24 rounded-full bg-gray-200"></div>
                        <div class="mb-2 h-4 w-32 bg-gray-200"></div>
                        <div class="h-3 w-24 bg-gray-200"></div>
                    </div>
                }
            }>
                {move || match user_data.get() {
                    None => view! { <div>"Loading..."</div> }.into_any(),
                    Some(None) => {
                        view! {
                            <div class="text-red-500">"Failed to load author information"</div>
                        }
                            .into_any()
                    }
                    Some(Some(user)) => {
                        let (role, _bio) = user.split_bio();
                        view! {
                            // Header Section
                            <div class="flex items-start space-x-4 rounded-lg [&:has(#profile:hover)]:bg-white [&:has(#profile:hover)]:shadow-lg">
                                <a rel="external" href=user.html_url.clone() class="flex-shrink-0">
                                    <img
                                        src=user.avatar_url
                                        alt="Author avatar"
                                        class="h-[4.5rem] w-[4.5rem] rounded-full border-2 border-gray-100"
                                    />
                                </a>
                                <div class="flex flex-1 flex-col gap-2 py-1.5">
                                    <a
                                        id="profile"
                                        rel="external"
                                        href=user.html_url.clone()
                                        class="group block -tracking-tight"
                                        title="GitHub profile"
                                    >
                                        <h4 class="text-lg leading-[1.2] font-bold text-[#202124] group-hover:text-black">
                                            {user.name.unwrap_or_else(|| "Unknown".to_string())}
                                        </h4>
                                        <p class="text-sm leading-[1.2] font-medium text-[#5f6368] group-hover:text-black">
                                            {role}
                                        </p>
                                    </a>

                                    // Social Links
                                    <div class="flex space-x-4">
                                        <a
                                            rel="external"
                                            href=user.html_url
                                            class="text-gray-400 transition-colors hover:text-gray-900"
                                            title="GitHub"
                                        >
                                            <GitHubIcon />
                                        </a>
                                        {move || {
                                            user.twitter_username
                                                .as_ref()
                                                .map(|username| {
                                                    view! {
                                                        <a
                                                            rel="external"
                                                            href=format!("https://twitter.com/{username}")
                                                            class="text-gray-400 transition-colors hover:text-gray-900"
                                                            title="Twitter"
                                                        >
                                                            <TwitterIcon />
                                                        </a>
                                                    }
                                                })
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
