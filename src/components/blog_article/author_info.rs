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
    pub location: Option<String>,
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
        <Suspense fallback=move || {
            view! {
                <div class="animate-pulse p-4">
                    <div class="h-24 w-24 rounded-full bg-gray-200 mb-4"></div>
                    <div class="h-4 w-32 bg-gray-200 mb-2"></div>
                    <div class="h-3 w-24 bg-gray-200"></div>
                </div>
            }
        }>
            {move || match user_data.get() {
                None => view! { <div>"Loading..."</div> }.into_any(),
                Some(None) => {
                    view! { <div class="text-red-500">"Failed to load author information"</div> }
                        .into_any()
                }
                Some(Some(user)) => {
                    let (role, bio) = user.split_bio();
                    view! {
                        <div class="gravatar-card bg-white rounded-lg shadow-lg overflow-hidden max-w-md">
                            <div class="p-6">
                                // Header Section
                                <div class="flex items-start space-x-4 mb-4">
                                    <a
                                        rel="external"
                                        href=user.html_url.clone()
                                        class="flex-shrink-0"
                                    >
                                        <img
                                            src=user.avatar_url
                                            alt="Author avatar"
                                            class="w-24 h-24 rounded-full border-2 border-gray-100"
                                        />
                                    </a>
                                    <div class="flex-1">
                                        <a
                                            rel="external"
                                            href=user.html_url.clone()
                                            class="block hover:text-blue-600"
                                        >
                                            <h4 class="text-xl font-semibold text-gray-900">
                                                {user.name.unwrap_or_else(|| "Unknown".to_string())}
                                            </h4>
                                            <p class="text-gray-600 mt-1">{role}</p>
                                            {move || {
                                                user.location
                                                    .as_ref()
                                                    .map(|location| {
                                                        view! {
                                                            <p class="text-gray-500 text-sm mt-1">
                                                                {location.to_owned()}
                                                            </p>
                                                        }
                                                    })
                                            }}
                                        </a>
                                    </div>
                                </div>

                                // Bio Section
                                <div class="mb-4">
                                    <p class="text-gray-700">{bio}</p>
                                </div>

                                // Social Links
                                <div class="flex space-x-4">
                                    <a
                                        rel="external"
                                        href=user.html_url
                                        class="text-gray-400 hover:text-gray-900 transition-colors"
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
                                                        class="text-gray-400 hover:text-gray-900 transition-colors"
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
    }
}
