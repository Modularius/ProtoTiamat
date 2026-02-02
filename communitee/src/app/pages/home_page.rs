use crate::{
    app::components::{AccessBar, AdColumns, Feed, MainColumn, ResourceView, SessionView},
    server::get_user_feed,
    structs::{PostData, UserData},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <SessionView fallback=move || view! {} action = |session| {
            let session = session.clone();
            let posts = Resource::new_blocking(||(), move|_| get_user_feed(session.user.clone(), 5));
            view!{
                <HomePageWithUser user_data = session.user_data posts />
            }
        } />
    }
}

#[component]
pub fn HomePageWithUser(
    user_data: UserData,
    posts: Resource<Result<Vec<PostData>, ServerFnError>>,
) -> impl IntoView {
    view! {
        <MainColumn>
            <h1> "Hi there " {user_data.name.clone()} "!" </h1>
            <AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "Current feed: "</h2>
                <ResourceView resource = posts
                    action = |posts| view!{<Feed feed = posts.into_iter() max = 10/>}
                />
            </AdColumns>
        </MainColumn>
    }
}
