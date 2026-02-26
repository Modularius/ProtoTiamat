use crate::{app::{
    TopLevelContext, components::{AdColumns, MainColumn, NewPostBox, PostBox, PostData}, generic_components::{IsLoggedIn, NotLoggedIn, RoundedBox, error_box}
}, structs::ContextExt};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
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
#[tracing::instrument]
pub async fn get_home_page_data(
    session_id: SessionUuid,
    max_posts: usize,
) -> Result<Option<HomePageData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect_context();
    let server = server_side_data.server.lock()?;
    
    let session = server.get_session(&session_id)
        .ok_or_else(||ServerFnErrorErr::ServerError(format!("No Session found with id {}", session_id.to_string())))?;

    let data = server.get_user(&session.user).map(|user| HomePageData {
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
    });
    Ok(data)
}

#[component]
#[tracing::instrument]
pub fn HomePage() -> impl IntoView {
    let home_page_data = ServerAction::new();
    view! {
        <MainColumn>
            <IsLoggedIn>
                {move || {
                    let session_id = use_context::<TopLevelContext>()
                        .expect_context()
                        .session_id
                        .get()
                        .expect("This must only be used in `IsLoggedIn` block, this should never fail.");
                    home_page_data.dispatch( GetHomePageData{ session_id, max_posts: 10 } );
                    Suspend::new(async move {
                        home_page_data.value().get().map(|home_page_data|
                            view!{
                                <ErrorBoundary fallback = error_box>
                                    {home_page_data.map(|home_page_data|
                                        home_page_data.map(|home_page_data|
                                            HomePageWithData(HomePageWithDataProps { home_page_data })
                                        )
                                    )}
                                </ErrorBoundary>
                            }
                        )
                    })
                }}
            </IsLoggedIn>
            <NotLoggedIn>
                <LandingPage />
            </NotLoggedIn>
        </MainColumn>
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
