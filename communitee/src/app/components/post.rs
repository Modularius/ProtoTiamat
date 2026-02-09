use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime, structs::{Post, User}};
    use chrono::Utc;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub author: String,
    pub author_link: String,
    pub datetime_posted: String,
    pub title: String,
    pub contents: String,
    pub replies: Vec<Self>,
}

#[cfg(feature = "ssr")]
impl PostData {
    pub fn new(post: &Post, author_user: &User) -> Self {
        Self {
            author: author_user.data.name.clone(),
            author_link: format!("/user/{}", author_user.data.id),
            datetime_posted: format_datetime(&post.data.posted_at),
            title: post.data.title.clone(),
            contents: post.data.content.clone(),
            replies: Default::default(),
        }
    }
}

#[component]
pub fn PostBox(post: PostData) -> impl IntoView {
    move || {
        let post = post.clone();
        view! {
            <div class = "post">
                <div class = "post-inner">
                    <div class = "header">
                        <div class = "user-name">
                            "Posted By: " <A exact = true href = {post.author_link}> {post.author} </A>
                        </div>
                        <div class = "datetime">
                            {post.datetime_posted}
                        </div>
                    </div>
                    <div class = "contents">
                        <div class = "contents-inner">
                            <div class = "post-title"> {post.title} </div>
                            {post.contents}
                        </div>
                    </div>
                    <div class = "replies">
                    </div>
                    <div class = "controls">
                        <div class = "button">Promote</div>
                        <div class = "button">Reply</div>
                        <div class = "button">Message</div>
                        <div class = "button">Block</div>
                    </div>
                </div>
            </div>
        }
    }
}
