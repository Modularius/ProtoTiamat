use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::app::generic_components::{ButtonControl, ButtonFunction, CloseButton, ControlStack};

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
            author_link: format!("/user/{}", author_user.data.id.to_string()),
            datetime_posted: format_datetime(&post.data.posted_at),
            title: post.data.title.clone(),
            contents: post.data.content.clone(),
            replies: post.replies.iter().map(|reply|Self::new(reply, author_user)).collect::<Vec<_>>(),
        }
    }
}

#[component]
fn PostContainer(children: Children) -> impl IntoView {
    view!{
        <div class = "flex flex-col
            bg-indigo-300 hover:bg-indigo-200
            w-11/12 md:w-4/5
            p-1 md:p-1
            m-4 md:m-4
        ">
        {children()}
        </div>
    }
}

#[component]
fn PostHeader(children: Children) -> impl IntoView {
    view!{
        <div class = "flex flex-row justify-even w-full p-1 m-1">
        {children()}
        </div>
    }
}

#[component]
fn PostTextField(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-indigo-400 p-1 m-1 text-left">
            {children()}
        </div>
    }
}

#[component]
fn PostMain(children: Children) -> impl IntoView {
    view!{
        <div class = "flex flex-col p-1 m-1">
            {children()}
        </div>
    }
}

#[component]
fn PostReplies(children: Children) -> impl IntoView {
    view!{
        <div class = "flex flex-col align-center p-1 m-1 bg-indigo-500 closable-container">
            {children()}
        </div>
    }
}

#[component]
pub fn PostBox(post: PostData) -> impl IntoView {
    let num_replies = post.replies.len();
    move || {
        let post = post.clone();
        view! {
            <PostContainer>
                <PostHeader>
                    <PostTextField>
                        "Posted By: " <A exact = true href = {post.author_link}> {post.author} </A>
                    </PostTextField>
                    <PostTextField>
                        {post.datetime_posted}
                    </PostTextField>
                </PostHeader>
                <PostMain>
                    <PostTextField>
                        {post.title}
                    </PostTextField>
                    <PostTextField>
                        {post.contents}
                    </PostTextField>
                </PostMain>

                <PostReplies>
                    <Show when = move||{num_replies>0} fallback = ||view!{
                        <div class = "w-full text-center italic"> "No Replies" </div>
                    } >
                        <CloseButton />
                        <div class = "closable">
                            <For
                                each = {
                                    let replies = post.replies.clone();
                                    move ||replies.clone().into_iter().enumerate()
                                }
                                key = |(i,_)|*i
                                children = |(_,reply)| PostBox(PostBoxProps{ post: reply })
                            />
                        </div>
                    </Show>
                </PostReplies>

                <ControlStack>
                    <ButtonControl value = "Promote" on_click = ButtonFunction::closure(|_ev|{}) />
                    <ButtonControl value = "Reply" on_click = ButtonFunction::closure(|_ev|{}) />
                    <ButtonControl value = "Message" on_click = ButtonFunction::closure(|_ev|{}) />
                    <ButtonControl value = "Block" on_click = ButtonFunction::closure(|_ev|{}) />
                </ControlStack>
            </PostContainer>
        }
    }
}
