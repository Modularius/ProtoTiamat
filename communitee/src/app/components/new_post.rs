use leptos::prelude::*;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime, structs::{Post, User}};
    use chrono::Utc;
} }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PostData {
//     pub author: String,
//     pub author_link: String,
//     pub datetime_posted: String,
//     pub title: String,
//     pub contents: String,
//     pub replies: Vec<Self>,
// }

#[component]
pub fn NewPostBox() -> impl IntoView {
    move || {
        view! {
            <div class = "post new-post">
                <div class = "post-inner new-post-inner">
                    <div class = "contents">
                        <div class = "contents-inner">
                            <div class = "post-title">
                                <label for = "new_post"> "Subject: " </label>
                                <input id = "new_post" type = "text" />
                            </div>
                            <label for = "post_content"> "Content: " </label>
                            <textarea id = "post_content"/>
                        </div>
                    </div>
                    <div>
                        <input type = "button" value = "Submit"/>
                    </div> 
                </div>
            </div>
        }
    }
}
