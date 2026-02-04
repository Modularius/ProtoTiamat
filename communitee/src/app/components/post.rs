use chrono::SubsecRound;
use leptos::prelude::*;

use crate::{app::components::ResourceView, server::get_user, structs::PostData};

#[component]
pub fn PostBox(post: PostData) -> impl IntoView {
    move || {
        let post = post.clone();
        let author_id = post.author.clone();
        let author = Resource::new_blocking(||(), move |_|get_user(author_id.clone()));
        view! {
            <div class = "post">
                <div class = "post-inner">
                    <div class = "header">
                        <div class = "user-name">
                            "Posted By: "
                            <a href = {format!("user/{}", post.author)}>
                                <ResourceView resource = author action = |author|author.map(|a|a.name) />
                            </a>
                        </div>
                        <div class = "datetime">
                            {
                                let date = post.posted_at.date_naive();
                                let time = post.posted_at.time().trunc_subsecs(0);
                                format!("On: {}, at: {}",
                                    date.to_string(),
                                    time.to_string()
                                )
                            }
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
}
