use abilitee::{
    ContextExt, Expect, TopLevelContext, app::{
        components::{AdColumns, LoginBox},
        generic_components::{ButtonControl, ButtonFunction, LabelledControlStack, SharpBox},
        guards::GuardedPage,
    }
};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
    use libertee::User;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FriendData {
    id: UserUuid,
    name: String,
    link: String,
}

#[cfg(feature = "ssr")]
impl FriendData {
    fn from(friend: &User) -> Self {
        Self {
            id: friend.data.id.clone(),
            name: friend.data.name.clone(),
            link: format!("/user/{}", friend.data.id.to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFriendslistPageDataContext {
    user_name: String,
    friends: Vec<FriendData>,
}

impl Expect for GetFriendslistPageDataContext {
    const EXPECT: &'static str = "";
}

#[server]
async fn get_friendslist_page_data(
    session_id: SessionUuid,
    max_friends: usize,
) -> Result<GetFriendslistPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = GetFriendslistPageDataContext {
        user_name: user.data.name.clone(),
        friends: user
            .data
            .friends
            .iter()
            .flatten()
            .take(max_friends)
            .flat_map(|friendship| {
                server
                    .get_user(&friendship.user_id)
                    .map(|friend| FriendData::from(friend))
            })
            .collect(),
    };

    Ok(data)
}

pub struct FriendlistPage;

impl GuardedPage for FriendlistPage {
    type DataContext = GetFriendslistPageDataContext;
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
    async fn fetch(_: Self::Source) -> Option<Result<GetFriendslistPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_friendslist_page_data(session_id, 10).await)
    }

    #[instrument]
    fn with_data() -> impl IntoView {
        let friendslist_page_data = use_context::<GetFriendslistPageDataContext>().expect_context();
        view! {
            <h1 class = "text-3xl m-6"> "Hi there " {friendslist_page_data.user_name} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> {format!("You have {} friend(s)", friendslist_page_data.friends.len())} </h2>
                <SharpBox>
                    <For
                        each = move ||friendslist_page_data.friends.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = move |(_,friend)| view!{
                            <LabelledControlStack label = {friend.name} href = {Some(friend.link)} class = "w-1/2">
                                <ButtonControl value = "Delegate" on_click = ButtonFunction::closure(|_|{}) />
                                <ButtonControl value = "Unfriend" on_click = ButtonFunction::closure(|_|{}) />
                                <ButtonControl value = "Block" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </SharpBox>
            </AdColumns>
        }
    }

    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            "Not Logged In"
            <LoginBox />
        }
    }
}