use crate::{
    Uuid,
    app::{components::{AdColumns, MainColumn, NewPostBox, PostBox, PostData}, generic_components::{ButtonControl, ControlStack, ResourceView, SessionView}},
    server_functions::format_datetime,
    structs::Session,
};
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

#[derive(Clone, Params, PartialEq)]
struct GroupParams {
    group_id: Option<String>,
}

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{Server, ServerSideData, structs::{Group, UserData, Member}};
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPageData {
    user_id: String,
    user_name: String,
    group_id: String,
    group_name: String,
    member: Option<GroupWithMemberPageData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupWithMemberPageData {
    datetime_joined: String,
    feed: Vec<PostData>,
    delegates: Vec<Option<GroupWithMemberDelegatePageData>>,
}

#[cfg(feature = "ssr")]
impl GroupWithMemberPageData {
    fn new(server: &Server, group: &Group, member: &Member) -> Self {
        Self {
            datetime_joined: format_datetime(&member.joined),
            feed: group.feed
                .posts
                .iter()
                .take(10)
                .flat_map(|post| {
                    server
                        .get_user(&post.data.author)
                        .map(|author_user| PostData::new(post, author_user))
                })
                .collect(),
            delegates: member
                .delegates
                .iter()
                .map(|(delegate_id, &weight)| {
                    group.data
                        .members
                        .get(delegate_id)
                        .and_then(|delegate|server.get_user(&delegate.user))
                        .map(|delegate|GroupWithMemberDelegatePageData
                            ::new(&delegate.data, weight)
                        )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GroupWithMemberDelegatePageData {
    name: String,
    link: String,
    weight: f64,
}

#[cfg(feature = "ssr")]
impl GroupWithMemberDelegatePageData {
    fn new(user_data: &UserData, weight: f64) -> Self {
        Self {
            name: user_data.name.clone(),
            link: format!("/user/{}", user_data.id),
            weight,
        }
    }
}

#[server]
pub async fn get_group_page_data(
    session: Session,
    group_id: Uuid,
) -> Result<GroupPageData, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;

    let group = server.get_group(&group_id);

    let data = GroupPageData {
        user_id: session.user_data.id,
        user_name: session.user_data.name,
        group_id: group_id.clone(),
        group_name: group
            .map(|group| group.data.name.clone())
            .unwrap_or("No Group".into()),
        member: group.and_then(|group| {
            group.data
                .members
                .get(&session.user)
                .map(|member|GroupWithMemberPageData::new(&server, group, member))
        }),
    };
    Ok(data)
}

#[component]
pub fn GroupPage() -> impl IntoView {
    || {
        view! {
            <SessionView action = |session: &Session| {
                let session = session.clone();
                let params = use_params::<GroupParams>();
                let group_id = move || {
                    params
                        .get()
                        .ok()
                        .and_then(move |p| p.group_id.clone())
                        .unwrap_or_default()
                };
                let group_page_data = {
                    let group_id = group_id.clone();
                    Resource::new_blocking(
                        move || (session.clone(), group_id()),
                        |(session,group_id)| get_group_page_data(session, group_id),
                    )
                };
                view!{
                    <ResourceView
                        resource = group_page_data
                        action = move |group_page_data|
                            GroupPageWithData(GroupPageWithDataProps{ group_page_data })
                    />
                }
            } />
        }
    }
}

#[component]
fn GroupPageWithData(group_page_data: GroupPageData) -> impl IntoView {
    let group_page_data = group_page_data.clone();
    let member = group_page_data.member;
    view! {
        <MainColumn>
            <h1 class = "text-4xl m-2"> "Hi there " {group_page_data.user_name} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2 class = "text-2xl m-2"> "Group Name: " {group_page_data.group_name} </h2>
                {
                    if let Some(member) = member{
                        Either::Left(view!{
                            <h3 class = "text-lg m-2"> "You joined on " {member.datetime_joined.clone()} "." </h3>
                            <DelegatePanel delegates = member.delegates.clone()/>
                            <h2 class = "text-lg m-2"> "Submit a post." </h2>
                            <NewPostBox user_id = group_page_data.user_id group_id = Some(group_page_data.group_id) />
                            <div class = "bg-indigo-700 m-4 p-2 rounded-2xl">
                                <h2 class = "text-lg m-2"> "Current Group Feed" </h2>
                                <For
                                    each = move ||member.feed.clone().into_iter().enumerate()
                                    key = |(i,_)|*i
                                    children = |(_,post)| view!{ <PostBox post/> }
                                />
                            </div>
                        })
                    } else {
                        Either::Right(view!{
                            <input type = "button" value = "Join this group?" />
                        })
                    }
                }
            </AdColumns>
        </MainColumn>
    }
}

#[component]
fn DelegatePanel(delegates: Vec<Option<GroupWithMemberDelegatePageData>>) -> impl IntoView {
    view!{
        <div class = "bg-indigo-700 m-4 p-2 rounded-2xl">
            <h3 class = "text-lg m-2"> "You have " {delegates.len()} " delegates(s) in this group." </h3>
            <ControlStack>
                <ButtonControl value = "Add New Delegate" on_click = |_|{} />
                <ButtonControl value = "Help on Delegates" on_click = |_|{} />
            </ControlStack>
            <For
                each=move||delegates.clone().into_iter().enumerate()
                key=|(i,_)|*i
                children=|(_,delegate)| Delegate(DelegateProps{ delegate })
            />
        </div>
    }
}

#[component]
fn Delegate(delegate: Option<GroupWithMemberDelegatePageData>) -> impl IntoView {
    if let Some(delegate) = delegate {
        Either::Left(view!{
            <div class = "bg-indigo-600 m-4 p-2 rounded-xl">
                <div class = "text-lg m-2">
                    <a href = {delegate.link}> {delegate.name} </a>: {delegate.weight}
                </div>
                <ControlStack>
                    <ButtonControl value = "Edit Weight" on_click = |_|{} />
                    <ButtonControl value = "Remove" on_click = |_|{} />
                </ControlStack>
            </div>
        })
    } else {
        Either::Right(view!{
            <div class = "bg-indigo-600 m-4 p-2 rounded-xl">
                <div> "Delegate not found." </div>
            </div>
        })
    }
}