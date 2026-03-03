use crate::{
    app::{
        components::{AdColumns, FootBar, MainColumn, NewPostBox, PostBox, PostData, TopBar},
        generic_components::RoundedBox,
        guards::{IsLoggedIn, NotLoggedIn, PageGuard, SessionGuard},
    },
    structs::{ContextExt, Expect},
};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime};
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
#[tracing::instrument]
pub async fn get_home_page_data(
    session_id: SessionUuid,
    max_posts: usize,
) -> Result<HomePageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)
        .map_err(ServerFnErrorErr::ServerError)?;

    let user = server
        .get_user(&session.user)
        .map_err(ServerFnErrorErr::ServerError)?;

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

#[component]
#[tracing::instrument]
pub fn HomePage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
            <MainColumn>
                <IsLoggedIn>
                    <PageGuard with_parameters = |session_id|GetHomePageData{ session_id, max_posts: 10 }>
                        <HomePageWithData />
                    </PageGuard>
                </IsLoggedIn>
                <NotLoggedIn>
                    <LandingPage />
                </NotLoggedIn>
            </MainColumn>
        <FootBar />
    </SessionGuard>
    }
}

#[component]
fn HomePageWithData() -> impl IntoView {
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

#[component]
fn LandingPage() -> impl IntoView {
    view! {
        <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
        <h2 class = "text-xl m-2"> "The social media platform exclusively controlled by its users." </h2>
        <RoundedBox>
            <h3 class = "text-lg m-2"> "Using Communitee guarantees:" </h3>
            <ul class = "text-sm m-2">
                <li> "Your content and data is *never* used to personalised your feed or the adverts you are shown." </li>
                <li> "Your experience is curated by yourself and fellow users, and never by an opaque algorithm controlled by tech companies." </li>
                <li> "You and your fellow users can anonymously vote for the content you like, and this vote exclusively determines which content is shown. There are no paid posts." </li>
                <li> "All adverts are clearly marked as adverts, and are chosen by the users." </li>
                <li> "Admins are democratically elected by the users they serve." </li>
                <li> "Content is moderated by fellow users who are empowered by the democratic wishes of the users they serve." </li>
                <li> "All users are verified in a safe and anonymous process, which guarantees identity without risking their private data." </li>
                <li> "Data is distributed among many cooperating nodes, with multiple levels of encryption to ensure privacy." </li>
            </ul>
        </RoundedBox>
    }
}
