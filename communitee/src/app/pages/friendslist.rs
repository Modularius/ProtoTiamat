use crate::{app::{
    components::{AdColumns, MainColumn},
    generic_components::{
        ButtonControl, ButtonFunction, LabelledControlStack, ResourceView, SessionView, SharpBox,
    },
}, structs::ContextExt};
use leptos::prelude::*;
use libertee::{Session, SessionUuid};
use libertee::UserUuid;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, structs::User};
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
pub struct FriendslistPageData {
    user_name: String,
    friends: Vec<FriendData>,
}

#[server]
async fn get_friendslist_page_data(
    session_id: SessionUuid,
    max_friends: usize,
) -> Result<FriendslistPageData, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect_context();
    let server = server_side_data.server.lock()?;

    let session = server.get_session(&session_id)
        .ok_or_else(||ServerFnErrorErr::ServerError(format!("No Session found with id {}", session_id.to_string())))?;

    let data = FriendslistPageData {
        user_name: session.user_data.name.clone(),
        friends: server
            .get_user(&session.user)
            .map(|user| {
                user.data
                    .friends
                    .iter()
                    .flatten()
                    .take(max_friends)
                    .flat_map(|friendship| {
                        server
                            .get_user(&friendship.user_id)
                            .map(|friend| FriendData::from(friend))
                    })
                    .collect()
            })
            .unwrap_or_default(),
    };

    Ok(data)
}

#[component]
pub fn FriendlistPage() -> impl IntoView {
    || {
        view! {
            <SessionView action = |session_id: SessionUuid| {
                let friendslist_page_data = Resource::new_blocking(
                    move ||session_id.clone(),
                    |session_id| get_friendslist_page_data(session_id, 10)
                );
                view!{
                    <ResourceView
                        resource = friendslist_page_data
                        action = |friendslist_page_data|
                            FriendlistPageWithData(FriendlistPageWithDataProps{ friendslist_page_data })
                    />
                }
            } />
        }
    }
}

#[component]
pub fn FriendlistPageWithData(friendslist_page_data: FriendslistPageData) -> impl IntoView {
    view! {
        <MainColumn>
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
        </MainColumn>
    }
}
