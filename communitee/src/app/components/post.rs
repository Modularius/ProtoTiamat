use leptos::prelude::*;

use crate::structs::PostData;

#[component]
pub fn PostBox(post: PostData) -> impl IntoView {
    view! {
        <div class = "post">
            <div class = "post-inner">
                <div class = "header">
                    <div class = "user-name">
                        "Posted By: "
                    </div>
                    <div class = "group">
                        "In: "
                    </div>
                    <div class = "datetime">
                        "On: "
                    </div>
                </div>
                <div class = "content">
                    <div class = "content-inner">
                        {post.content}
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
