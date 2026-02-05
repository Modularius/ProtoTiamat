use crate::{
    app::components::{AccessBar, AdColumns, Feed, MainColumn, ResourceView, SessionView},
    server::get_user_feed,
    structs::{PostData, UserData},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    || view! {
        <SessionView fallback=move || view! {} action = |session| {
            let session = session.clone();
            let user_feed_data = Resource::new_blocking(move ||session.user.clone(), |user_id| get_user_feed(user_id, 5));
            view!{
                <MainColumn>
                    <h1> "Hi there " {session.user_data.name.clone()} "!" </h1>
                    //<AccessBar user_data = user_data.clone()/>
                    <AdColumns>
                        <div>
                        <ResourceView resource = user_feed_data
                            action = |user_feed_data| user_feed_data.map(|user_feed_data|
                                view!{
                                    <h2> "Current feed (as of " {user_feed_data.datetime_feed_generated} "): "</h2>
                                    <Feed feed = user_feed_data.posts.into_iter() max = 10/>
                                }
                            )
                        />
                        </div>
                    </AdColumns>
                </MainColumn>
            }
        } />
    }
}
