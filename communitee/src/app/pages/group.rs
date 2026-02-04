use crate::{
    app::components::{AdColumns, MainColumn, ResourceView, SessionView}, server::{get_group, get_group_and_member, get_group_member}, structs::{GroupData, Member, Session, UserData}
};
use chrono::SubsecRound;
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct GroupParams {
    group_id: Option<String>,
}

#[component]
pub fn GroupPage() -> impl IntoView {
    || view! {
        <SessionView action = |session: &Session| {
            let session = session.clone();
            view!{ <GroupsPageWithUser user_data = session.user_data /> }
        } />
    }
}

#[component]
pub fn GroupsPageWithUser(user_data: UserData) -> impl IntoView {
    move || {
        let user_data = user_data.clone();
        let params = use_params::<GroupParams>();
        let group_id = params.read()
            .as_ref()
            .ok()
            .and_then(move |p|p.group_id.clone())
            .unwrap_or_default();

        let group_data = {
            let group_id = group_id.clone();
            Resource::new_blocking(move || group_id.clone(), move|group_id| get_group(group_id))
        };
        let group_member = {
            let group_id = group_id.clone();
            Resource::new_blocking(
                move ||(group_id.clone(), user_data.id.clone()),
                move|(group_id, user_id)| get_group_member(group_id, user_id)
            )
        };
        view! {
            <MainColumn>
                <h1> "Hi there " {user_data.name.clone()} "!" </h1>
                //<AccessBar user_data = user_data.clone()/>
                <AdColumns>
                    <h2> "Groups you are currently subscribed to or following: "</h2>
                    <ResourceView
                        resource = group_data
                        action = move |group_data|
                            view!{<GroupsPageWithUserAndGroup group_data = group_data group_member = group_member />}
                    />
                </AdColumns>
            </MainColumn>
        }
    }
}

#[component]
fn GroupsPageWithUserAndGroup(group_data: Option<GroupData>, group_member: Resource<Result<Option<Member>, ServerFnError>>) -> impl IntoView {
    match group_data {
        Some(group_data) => Either::Left(view! {
            <h2> "Group Name: " {group_data.name.clone()} </h2>
            <ResourceView
                resource = group_member
                action = move |group_member|
                    view!{<GroupsPageWithUserAndGroupAndMember group_data = group_data.clone() group_member = group_member />}
            />
        }),
        None => Either::Right(view! {
            Missing Group
        })
    }
}

#[component]
fn GroupsPageWithUserAndGroupAndMember(group_data: GroupData, group_member: Option<Member>) -> impl IntoView {
    match group_member {
        Some(group_member) => Either::Left(view! {
            <h3> "You joined on " {
                let date = group_member.joined.date_naive();
                let time = group_member.joined.time().trunc_subsecs(0);
                format!("On: {}, at: {}",
                    date.to_string(),
                    time.to_string()
                )
            }. </h3>
            "You have " {group_member.delegates.len()} " delegates(s) in this group."
        }),
        None => Either::Right(view! {
            Missing Group
        })
    }
}