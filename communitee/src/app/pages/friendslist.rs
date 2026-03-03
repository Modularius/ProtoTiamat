use crate::{
    app::{
        components::{AdColumns, FootBar, MainColumn, TopBar},
        generic_components::{ButtonControl, ButtonFunction, LabelledControlStack, SharpBox},
        guards::{IsLoggedIn, NotLoggedIn, PageGuard, SessionGuard},
    },
    structs::{ContextExt, Expect},
};
use leptos::prelude::*;
use libertee::{LiberteeError, SessionUuid, UserUuid};
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
        .get_session(&session_id)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    let user = server
        .get_user(&session.user)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

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

#[component]
pub fn FriendlistPage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
                <IsLoggedIn>
                    <PageGuard with_parameters = |session_id|GetFriendslistPageData{ session_id, max_friends: 10 }>
                        <FriendlistPageWithData />
                    </PageGuard>
                </IsLoggedIn>

                <NotLoggedIn>
                    "Not Logged In"
                </NotLoggedIn>
            <FootBar/>
        </SessionGuard>
    }
}

#[component]
pub fn FriendlistPageWithData() -> impl IntoView {
    let friendslist_page_data = use_context::<GetFriendslistPageDataContext>().expect_context();
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
