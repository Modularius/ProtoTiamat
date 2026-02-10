use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::app::generic_components::{Control, ControlStack};

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
            <div class = "flex flex-col
                bg-indigo-300 hover:bg-indigo-200
                w-11/12 md:w-4/5
                p-1 md:p-1
                m-4 md:m-4
            ">
                <div class = "flex flex-row justify-even w-full p-1 m-1">
                    <div class = "bg-indigo-400
                        p-1 m-1
                        text-left
                    ">
                        "Posted By: " <A exact = true href = {post.author_link}> {post.author} </A>
                    </div>
                    <div class = "bg-indigo-400
                        p-1 m-1
                        text-left
                    ">
                        {post.datetime_posted}
                    </div>
                </div>
                <div class = "flex flex-col p-1 m-1">
                    <div class = "bg-indigo-400 p-1 text-left"> {post.title} </div>
                    <div class = "bg-indigo-400 p-1 text-left">
                        {post.contents}
                    </div>
                </div>
                <div class = "p-1 m-1 bg-indigo-500">
                </div>
                <ControlStack>
                    <Control>
                        <input type = "button" value = "Promote" />
                    </Control>
                    <Control>
                        <input type = "button" value = "Reply" />
                    </Control>
                    <Control>
                        <input type = "button" value = "Message" />
                    </Control>
                    <Control>
                        <input type = "button" value = "Block" />
                    </Control>
                </ControlStack>
            </div>
        }
    }
}
