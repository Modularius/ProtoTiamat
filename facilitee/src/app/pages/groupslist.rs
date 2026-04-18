use abilitee::{
    ContextExt, Expect, TopLevelContext, app::{
        components::{AdColumns, LoginBox},
        generic_components::{ButtonControl, ButtonFunction, LabelledControlStack, RoundedBox},
        guards::GuardedPage,
    }
};
use leptos::prelude::*;
use libertee::{GroupData, SessionUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupslistPageDataContext {
    user_name: String,
    groups: Vec<GroupData>,
}

impl Expect for GroupslistPageDataContext {
    const EXPECT: &'static str =
        "GroupslistPageDataContext should be provided, this should never fail.";
}

impl Default for GroupslistPageDataContext {
    fn default() -> Self {
        Self {
            user_name: "User Unknown".into(),
            groups: Default::default(),
        }
    }
}

#[server]
pub async fn get_groupslist_page_data(
    session_id: SessionUuid,
    max_groups: usize,
) -> Result<GroupslistPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = GroupslistPageDataContext {
        user_name: user.data.name.clone(),
        groups: user
            .data
            .groups
            .iter()
            .flatten()
            .take(max_groups)
            .flat_map(|group_id| server.get_group(group_id).map(|group| group.data.clone()))
            .collect(),
    };
    Ok(data)
}


pub struct GroupslistPage;

impl GuardedPage for GroupslistPage {
    type DataContext = GroupslistPageDataContext;
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
    async fn fetch(_: Self::Source) -> Option<Result<GroupslistPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_groupslist_page_data(session_id, 10).await)
    }

    #[instrument]
    fn with_data() -> impl IntoView {
        let groupslist_page_data = use_context::<GroupslistPageDataContext>().expect_context();
        view! {
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
                            <LabelledControlStack label = {group.name} href = {Some(format!("/group/{}", group.id))} class = "w-1/2">
                                <ButtonControl value = "Unsubscribe" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </RoundedBox>
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