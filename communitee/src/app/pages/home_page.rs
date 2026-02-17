use crate::app::{
    components::{AdColumns, Feed, MainColumn, NewPostBox, PostBox, PostData},
    generic_components::{ResourceView, RoundedBox, SessionView},
};
use leptos::prelude::*;
use libertee::{Session, UserUuid};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime};
    use chrono::Utc;
} }

#[derive(Clone, Serialize, Deserialize)]
pub struct HomePageData {
    user_id: UserUuid,
    user_name: String,
    datetime_feed_generated: String,
    posts: Vec<PostData>,
}

#[server]
pub async fn get_home_page_data(
    session: Session,
    max_posts: usize,
) -> Result<Option<HomePageData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;

    let data = server.get_user(&session.user).map(|user| HomePageData {
        user_id: user.data.id.clone(),
        user_name: session.user_data.name.clone(),
        datetime_feed_generated: format_datetime(&Utc::now()),
        posts: user
            .store
            .posts
            .iter()
            .take(max_posts)
            .flat_map(|(_, post)| {
                server
                    .get_user(&post.data.author)
                    .map(|author_user| PostData::new(post, author_user))
            })
            .collect(),
    });
    Ok(data)
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <SessionView action = |session| {
            let session = session.clone();
            let home_page_data = Resource::new_blocking(
                move ||session.clone(),
                |session| get_home_page_data(session, 10)
            );
            view!{
                <ResourceView
                    resource = home_page_data
                    action = |home_page_data| home_page_data
                        .map(|home_page_data|HomePageWithData(HomePageWithDataProps{home_page_data}))
                />
            }
        } />
    }
}

#[component]
fn HomePageWithData(home_page_data: HomePageData) -> impl IntoView {
    /*let posts = RwSignal::new(home_page_data
        .posts
        .into_iter()
        .map(RwSignal::new)
        .collect::<Vec<_>>()
    );
    let datetime_feed_generated = RwSignal::new(home_page_data
        .datetime_feed_generated
    );*/
    view! {
        <MainColumn>
            <h1 class = "text-3xl m-6"> "Hi there " {home_page_data.user_name.clone()} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <RoundedBox>
                    <h2 class = "text-xl m-2"> "Submit a post:" </h2>
                    <NewPostBox
                        user_id = {home_page_data.user_id.clone()}
                        group_id = None
                    />
                </RoundedBox>
                <RoundedBox>
                    <h2 class = "text-lg m-2"> "Current feed (as of " {home_page_data.datetime_feed_generated.clone()} "): "</h2>
                    <For
                        each = move ||home_page_data.posts.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,post)| view!{
                            <PostBox post = post/>
                        }
                    />
                </RoundedBox>
            </AdColumns>
        </MainColumn>
    }
}
