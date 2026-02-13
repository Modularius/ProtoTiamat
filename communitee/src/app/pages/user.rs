use std::collections::HashMap;

use crate::{
    Uuid,
    app::{components::{AdColumns, MainColumn}, generic_components::{ButtonControl, ControlStack, ResourceView, SessionView}},
};
use leptos::{Params, either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, server_functions::format_datetime};
    use chrono::Utc;
} }

#[derive(Clone, Params, PartialEq)]
struct UserParams {
    user_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPageData {
    pub name: String,
    pub datetime_joined: String,
    pub properties: HashMap<String, String>,
    pub groups_in: Vec<GroupInData>,
    pub friends: Vec<FriendOfData>,
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
async fn get_user_page_data(user_id: Option<Uuid>) -> Result<Option<UserPageData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;

    let user_page_data = user_id
        .and_then(|user_id| server.get_user(&user_id))
        .map(|user| {
            let properties = user.data.properties.clone();
            let groups_in = user
                .data
                .groups
                .iter()
                .flat_map(|group_id| {
                    server.get_group(group_id).and_then(|group| {
                        group
                            .data
                            .members
                            .get(&user.data.id)
                            .map(|member| GroupInData {
                                name: group.data.name.clone(),
                                link_to_group: format!("/group/{}", group.data.id),
                                datetime_joined: format_datetime(&member.joined),
                            })
                    })
                })
                .collect();
            let friends = user
                .data
                .friends
                .iter()
                .flat_map(|friendship| {
                    server.get_user(&friendship.user_id).map(|friend| FriendOfData {
                        name: friend.data.name.clone(),
                        link_to_user: format!("/user/{}", friend.data.id),
                        datetime_of_friendship: format_datetime(&friendship.datetime_of_friendship),
                    })
                })
                .collect();

            UserPageData {
                name: user.data.name.clone(),
                datetime_joined: format_datetime(&user.data.datetime_joined),
                properties,
                groups_in,
                friends,
            }
        });
    Ok(user_page_data)
}

#[component]
pub fn UserPage() -> impl IntoView {
    {
        view! {
            <SessionView action = |session| {
                let session = session.clone();
                let params = use_params::<UserParams>();
                let user_id = move || params.get()
                    .ok()
                    .and_then(|params|params.user_id);

                let user_page_data = Resource::new_blocking(
                    user_id,
                    |user_id| get_user_page_data(user_id.clone())
                );
                view!{
                    <MainColumn>
                        <h1> "Hi there " {session.user_data.name.clone()} "!" </h1>
                        //<AccessBar user_data = user_data.clone()/>
                        <AdColumns>
                            <div>
                            <ResourceView resource = user_page_data action = move |user_page_data| {
                                match user_page_data {
                                    Some(user_page_data) => Either::Left(view!{
                                        <UserPageWithData user_page_data />
                                    }),
                                    None => Either::Right(view!{}),
                                }
                            }/>
                            </div>
                        </AdColumns>
                    </MainColumn>
                }
            } />
        }
    }
}

#[component]
pub fn UserPageWithData(user_page_data: UserPageData) -> impl IntoView {
    view! {
        <h2> "Communitee User: " {user_page_data.name} </h2>
        <h3> "Joined Communitee on: " {user_page_data.datetime_joined} </h3>
        <div class = "bg-indigo-700 m-4 p-2 rounded-2xl">
            <h3> "Has " {user_page_data.friends.len()} " friend(s)." </h3>
            <For
                each = move ||user_page_data.friends.clone().into_iter().enumerate()
                key = |(i,_)|*i
                children = |(_,friend)| view!{
                    <ControlStack>
                        <a href = {friend.link_to_user}> {friend.name} </a>
                        <ButtonControl value = "Block User" on_click = |_|{} />
                        <ButtonControl value = "Add/Remove Friend" on_click = |_|{} />
                    </ControlStack>
                }
            />
        </div>

        <div class = "bg-indigo-700 m-4 p-2 rounded-2xl">
            <h3> "Is subscribed to " {user_page_data.groups_in.len()} " group(s)." </h3>
            <For
                each = move ||user_page_data.groups_in.clone().into_iter().enumerate()
                key = |(i,_)|*i
                children = |(_,group)| view!{
                    <ControlStack>
                        <a href = {group.link_to_group}> {group.name} </a>
                        <ButtonControl value = "Join Group" on_click = |_|{} />
                    </ControlStack>
                }
            />
        </div>
    }
}
