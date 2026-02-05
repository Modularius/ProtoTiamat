use leptos::prelude::*;

use crate::structs::PostData;

#[component]
pub fn PostBox(post: PostData) -> impl IntoView {
    move || {
        let post = post.clone();
        view! {
            <div class = "post">
                <div class = "post-inner">
                    <div class = "header">
                        <div class = "user-name">
                            "Posted By: " <a href = {post.author_link}> {post.author} </a>
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
