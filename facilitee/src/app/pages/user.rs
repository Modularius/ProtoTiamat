use std::collections::HashMap;

use abilitee::{
    ContextExt, Expect, TopLevelContext,
    app::{
        components::{AdColumns, HelpBox, LoginBox},
        generic_components::{ButtonControl, ButtonFunction, LabelledControlStack, RoundedBox},
        guards::{GuardedComponentWithResource, GuardedComponentWithoutSession, GuardedPage},
    },
};
use leptos::{Params, prelude::*};
use leptos_router::{hooks::use_params, params::{Params, ParamsError}};
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use abilitee::ServerSideData;
    }
}

#[derive(Clone, Debug, Params, PartialEq)]
pub struct UserParams {
    user_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPageDataContext {
    pub current_user_name: String,
    pub this_page_user_name: String,
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
    
    let session = server
        .get_session(&session_id)?;

    let current_user = server
        .get_user(&session.user)?;

    let this_page_user = server
        .get_user(&user_id)?;

    let properties = this_page_user.data.properties.clone();
    
    let groups_in = this_page_user
        .data
        .groups
        .iter()
        .flatten()
        .flat_map(|group_id| {
            server.get_group(group_id).ok().and_then(|group| {
                let member_id = group.get_member_id_from_user_id(&this_page_user.data.id);
                member_id
                    .and_then(|member_id| group.data.members.get(member_id))
                    .map(|_member| GroupInData {
                        name: group.data.name.clone(),
                        link_to_group: format!("/group/{}", group.data.id.to_string()),
                        datetime_joined: chrono::Utc::now().to_rfc3339(), //format_datetime(&member.joined),
                    })
            })
        })
        .collect();
    let friends = this_page_user
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
                    datetime_of_friendship: chrono::Utc::now().to_rfc3339(),
                })
        })
        .collect();

    Ok(UserPageDataContext {
        current_user_name: current_user.data.name.clone(),
        this_page_user_name: this_page_user.data.name.clone(),
        datetime_joined: chrono::Utc::now().to_rfc3339(), //format_datetime(&user.data.datetime_joined),
        properties: properties.unwrap_or_default(),
        groups_in,
        friends,
    })
}

pub struct UserPage;

impl GuardedComponentWithResource for UserPage {
    type DataContext = UserPageDataContext;
    type Source = (usize, usize, Result<UserParams, ParamsError>);
    
    #[instrument]
    fn source() -> Self::Source {
        let params = use_params::<UserParams>();
        let top_level_context = use_context::<TopLevelContext>().expect_context();
        (
            top_level_context.login.version().get(),
            top_level_context.logout.version().get(),
            params.get(),
        )
    }

    #[instrument]
    async fn fetch((_, _, params): Self::Source) -> Option<Result<UserPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        match params {
            Ok(up) => match up.user_id {
                Some(id) => Some(get_user_page_data(session_id, UserUuid(id)).await),
                None => None,
            },
            Err(_) => None,
        }
    }

    #[instrument]
    fn with_session_and_resource() -> impl IntoView {
        let user_page_data = use_context::<UserPageDataContext>().expect_context();
        view! {
            <h1> "Hi there " {user_page_data.current_user_name} "!" </h1>
            <AdColumns>
                <h2> "Communitee User: " {user_page_data.this_page_user_name} </h2>
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
        }
    }
}

impl GuardedComponentWithoutSession for UserPage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
            <HelpBox />
            <LoginBox />
        }
    }
}

impl GuardedPage for UserPage {}