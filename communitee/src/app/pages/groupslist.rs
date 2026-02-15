use crate::app::{
    components::{AdColumns, MainColumn},
    generic_components::{
        ButtonControl, ButtonFunction, LabelledControlStack, ResourceView, RoundedBox, SessionView,
    },
};
use leptos::prelude::*;
use libertee::{GroupData, Session};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupslistPageData {
    user_name: String,
    groups: Vec<GroupData>,
}

impl Default for GroupslistPageData {
    fn default() -> Self {
        Self {
            user_name: "User Unknown".into(),
            groups: Default::default(),
        }
    }
}

#[server]
pub async fn get_groupslist_page_data(
    session: Session,
    max_groups: usize,
) -> Result<GroupslistPageData, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    let data = server
        .get_user(&session.user)
        .map(|user| GroupslistPageData {
            user_name: session.user_data.name.clone(),
            groups: user
                .data
                .groups
                .iter()
                .take(max_groups)
                .flat_map(|group_id| server.get_group(group_id).map(|group| group.data.clone()))
                .collect(),
        })
        .unwrap_or_default();
    Ok(data)
}

#[component]
pub fn GroupslistPage() -> impl IntoView {
    || {
        view! {
            <SessionView action = |session: &Session| {
                let session = session.clone();
                let groupslist_page_data = {
                    let session = session.clone();
                    Resource::new_blocking(
                        move || session.clone(),
                        |session| get_groupslist_page_data(session, 5),
                    )
                };
                view!{
                    <ResourceView
                        resource = groupslist_page_data
                        action = |groupslist_page_data|
                            GroupslistPageWithData(GroupslistPageWithDataProps{ groupslist_page_data })
                    />
                }
            } />
        }
    }
}

#[component]
pub fn GroupslistPageWithData(groupslist_page_data: GroupslistPageData) -> impl IntoView {
    view! {
        <MainColumn>
            <h1> "Hi there " {groupslist_page_data.user_name} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "Groups you are currently subscribed to or following: "</h2>
                <div> "You have membership of " {groupslist_page_data.groups.len()} " group(s)" </div>
                <RoundedBox>
                    <For
                        each = move ||groupslist_page_data.groups.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,group)| view!{
                            <LabelledControlStack label = {group.name} href = {Some(format!("/group/{}", group.id.to_string()))} class = "w-1/2">
                                <ButtonControl value = "Unsubscribe" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </RoundedBox>
            </AdColumns>
        </MainColumn>
    }
}
