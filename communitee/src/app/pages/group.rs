use crate::{
    Uuid, app::components::{AdColumns, MainColumn, ResourceView, SessionView}, server_functions::{format_datetime, get_group, get_group_and_member, get_group_member}, structs::{GroupData, Member, Session, UserData}
};
use chrono::SubsecRound;
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

#[derive(Clone, Params, PartialEq)]
struct GroupParams {
    group_id: Option<String>,
}

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPageData {
    user_name: String,
    group_name: String,
    member: Option<GroupWithMemberPageData>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GroupWithMemberPageData {
    datetime_joined: String,
    delegates: Vec<String>,
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
        user_name: session.user_data.name,
        group_name: group.map(|group|group.data.name.clone()).unwrap_or("No Group".into()),
        member: group.and_then(|group| group.data
            .members
            .get(&session.user)
            .map(|member| GroupWithMemberPageData {
                datetime_joined: format_datetime(&member.joined),
                delegates: member.delegates
                    .iter()
                    .map(|(delegate_id, weight)|
                        format!("")
                    )
                    .collect()
                }
            )
        )
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
pub fn GroupPageWithData(group_page_data: GroupPageData) -> impl IntoView {
    move || {
        let group_page_data = group_page_data.clone();
        let member = group_page_data.member.unwrap();
        view! {
            <MainColumn>
                <h1> "Hi there " {group_page_data.user_name} "!" </h1>
                //<AccessBar user_data = user_data.clone()/>
                <AdColumns>
                    <h2> "Group Name: " {group_page_data.group_name} </h2>
                    <h3> "You joined on " {member.datetime_joined} "." </h3>
                    "You have " {member.delegates.len()} " delegates(s) in this group."
                </AdColumns>
            </MainColumn>
        }
    }
}

#[component]
fn GroupsPageWithUserAndGroup(
    group_data: Option<GroupData>,
    group_member: Resource<Result<Option<Member>, ServerFnError>>,
) -> impl IntoView {
    match group_data {
        Some(group_data) => Either::Left(view! {
            <ResourceView
                resource = group_member
                action = move |group_member|
                    view!{<GroupsPageWithUserAndGroupAndMember group_data = group_data.clone() group_member = group_member />}
            />
        }),
        None => Either::Right(view! {
            Missing Group
        }),
    }
}

#[component]
fn GroupsPageWithUserAndGroupAndMember(
    group_data: GroupData,
    group_member: Option<Member>,
) -> impl IntoView {
    match group_member {
        Some(group_member) => Either::Left(view! {
        }),
        None => Either::Right(view! {
            Missing Group
        }),
    }
}
