use std::collections::HashMap;

use crate::{
    app::{
        TopLevelContext,
        components::{AdColumns, FootBar, MainColumn, TopBar},
        generic_components::{ButtonControl, ButtonFunction, LabelledControlStack, RoundedBox},
        guards::{IsLoggedIn, PageGuard, ResourceGuard, SessionGuard},
    },
    structs::{ContextExt, Expect},
};
use leptos::{Params, either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use libertee::{LiberteeError, SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime};
} }

#[derive(Clone, Params, PartialEq)]
struct UserParams {
    user_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPageDataContext {
    pub user_name: String,
    pub name: String,
    pub datetime_joined: String,
    pub properties: HashMap<String, String>,
    pub groups_in: Vec<GroupInData>,
    pub friends: Vec<FriendOfData>,
}

impl Expect for UserPageDataContext {
    const EXPECT: &'static str = "UserPageDataContext should be provided, this should never fail.";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupInData {
    pub name: String,
    pub link_to_group: String,
    pub datetime_joined: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendOfData {
    pub name: String,
    pub link_to_user: String,
    pub datetime_of_friendship: String,
}

#[server]
async fn get_user_page_data(
    session_id: SessionUuid,
    user_id: UserUuid,
) -> Result<UserPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;
    tracing::debug!("Got Server");
    let session = server
        .get_session(&session_id)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    tracing::debug!("Got Session");
    let user = server
        .get_user(&user_id)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    tracing::debug!("Got User");
    let properties = user.data.properties.clone();
    tracing::debug!("Got Props");
    let groups_in = user
        .data
        .groups
        .iter()
        .flatten()
        .flat_map(|group_id| {
            server.get_group(group_id).ok().and_then(|group| {
                let member_id = group.get_member_id_from_user_id(&user.data.id);
                member_id
                    .and_then(|member_id| group.data.members.get(member_id))
                    .map(|member| GroupInData {
                        name: group.data.name.clone(),
                        link_to_group: format!("/group/{}", group.data.id.to_string()),
                        datetime_joined: chrono::Utc::now().to_rfc3339(), //format_datetime(&member.joined),
                    })
            })
        })
        .collect();
    let friends = user
        .data
        .friends
        .iter()
        .flatten()
        .flat_map(|friendship| {
            server
                .get_user(&friendship.user_id)
                .map(|friend| FriendOfData {
                    name: friend.data.name.clone(),
                    link_to_user: format!("/user/{}", friend.data.id.to_string()),
                    datetime_of_friendship: chrono::Utc::now().to_rfc3339(), // format_datetime(&friendship.datetime_of_friendship),
                })
        })
        .collect();

    Ok(UserPageDataContext {
        user_name: user.data.name.clone(),
        name: user.data.name.clone(),
        datetime_joined: chrono::Utc::now().to_rfc3339(), //format_datetime(&user.data.datetime_joined),
        properties: properties.unwrap_or_default(),
        groups_in,
        friends,
    })
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserPageParamsContext {
    user_id: UserUuid,
}

impl Expect for UserPageParamsContext {
    const EXPECT: &'static str =
        "UserPageParamsContext should be provided, this should never fail.";
}

#[component]
pub fn UserPage() -> impl IntoView {
    let params = use_params::<UserParams>();
    let source = move || {
        let tlc = use_context::<TopLevelContext>().expect_context();
        (
            tlc.login.version().get(),
            tlc.logout.version().get(),
            params.get(),
        )
    };
    let fetch = async |(_, _, params): (_, _, Result<UserParams, _>)| {
        let session_id: SessionUuid = use_context::<TopLevelContext>()
            .expect_context()
            .login_expect();
        match params {
            Ok(up) => match up.user_id {
                Some(id) => Some(get_user_page_data(session_id, UserUuid(id)).await),
                None => None,
            },
            Err(_) => None,
        }
    };
    let resource = Resource::new(source, fetch);
    view! {
    <SessionGuard>
        <TopBar/>
            <IsLoggedIn>
                    <ResourceGuard resource>
                        <UserPageWithData />
                    </ResourceGuard>
                    /*<PageGuard with_parameters = |session_id| GetUserPageData{
                            session_id,
                            user_id: { use_context::<UserPageParamsContext>().expect_context().user_id }
                        }>
                        <UserPageWithData />
                    </PageGuard>*/
            </IsLoggedIn>
        <FootBar/>
    </SessionGuard>
            }
    /*
    || {
        let params = use_params::<UserParams>();
        let user_id = params
            .get()
            .ok()
            .and_then(|params| params.user_id.map(UserUuid));
        match user_id {
            Some(user_id) => Either::Left({
                provide_context(UserPageParamsContext { user_id });

            }),
            None => Either::Right(view! {
                <SessionGuard>
                    <TopBar/>
                        <MainColumn>
                            <div> "No User Found" </div>
                        </MainColumn>
                    <FootBar/>
                </SessionGuard>
            }),
        }
    } */
}

#[component]
pub fn UserPageWithData() -> impl IntoView {
    let user_page_data = use_context::<UserPageDataContext>().expect_context();
    view! {
        <MainColumn>
            <h1> "Hi there " {user_page_data.user_name} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "Communitee User: " {user_page_data.name} </h2>
                <h3> "Joined Communitee on: " {user_page_data.datetime_joined} </h3>
                <RoundedBox>
                    <h3> "Has " {user_page_data.friends.len()} " friend(s)." </h3>
                    <For
                        each = move ||user_page_data.friends.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,friend)| view!{
                            <LabelledControlStack label = {friend.name} href = {Some(friend.link_to_user)} class = "w-1/2">
                                <ButtonControl value = "Block User" on_click = ButtonFunction::closure(|_|{}) />
                                <ButtonControl value = "Add/Remove Friend" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </RoundedBox>

                <RoundedBox>
                    <h3> "Is subscribed to " {user_page_data.groups_in.len()} " group(s)." </h3>
                    <For
                        each = move ||user_page_data.groups_in.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,group)| view!{
                            <LabelledControlStack label = {group.name} href = {Some(group.link_to_group)} class = "w-1/2">
                                <ButtonControl value = "Join Group" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </RoundedBox>
            </AdColumns>
        </MainColumn>
    }
}
