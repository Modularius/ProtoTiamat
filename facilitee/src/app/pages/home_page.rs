use abilitee::{
    ContextExt, Expect,
    app::{
        TopLevelContext,
        components::{
            AdColumns, HelpBox, LoginBox, NewPostBox, PostBox, PostData
        },
        generic_components::RoundedBox, guards::{GuardedComponentWithResource, GuardedComponentWithoutSession, GuardedPage},
    },
};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use abilitee::{ServerSideData, format_datetime};
    use chrono::Utc;
} }

#[derive(Clone, Serialize, Deserialize)]
pub struct HomePageDataContext {
    user_id: UserUuid,
    user_name: String,
    datetime_feed_generated: String,
    posts: Vec<PostData>,
}

impl Expect for HomePageDataContext {
    const EXPECT: &'static str = "HomePageDataContext should be provided, this should never fail.";
}

#[server]
#[instrument]
pub async fn get_home_page_data(
    session_id: SessionUuid,
    max_posts: usize,
) -> Result<HomePageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server.get_session(&session_id)?;

    let user = server.get_user(&session.user)?;

    let data = HomePageDataContext {
        user_id: user.data.id.clone(),
        user_name: user.data.name.clone(),
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
    };
    Ok(data)
}

pub struct HomePage;

impl GuardedComponentWithResource for HomePage {
    type DataContext = HomePageDataContext;
    type Source = (usize, usize);
    
    #[instrument]
    fn source() -> Self::Source {
        let top_level_context = use_context::<TopLevelContext>().expect_context();
        (
            top_level_context.login.version().get(),
            top_level_context.logout.version().get(),
        )
    }

    #[instrument]
    async fn fetch(_: Self::Source) -> Option<Result<HomePageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_home_page_data(session_id, 10).await)
    }

    #[instrument]
    fn with_session_and_resource() -> impl IntoView {
        let home_page_data = use_context::<HomePageDataContext>().expect_context();
        view! {
            <h1 class = "text-3xl m-6"> "Hi there " {home_page_data.user_name.clone()} "!" </h1>
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
        }
    }
}

impl GuardedComponentWithoutSession for HomePage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
            <HelpBox />
            <LoginBox />
        }
    }
}

impl GuardedPage for HomePage {}