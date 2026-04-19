use abilitee::{
    ContextExt, Expect, TopLevelContext, app::{
        components::{AdColumns, LoginBox},
        generic_components::RoundedBox,
        guards::{GuardedComponentWithResource, GuardedComponentWithoutSession, GuardedPage},
    }
};
use leptos::prelude::*;
use libertee::SessionUuid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MessageData {
    from: (String, String),
    to: Vec<(String, String)>,
    subject: String,
    content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessagesPageDataContext {
    user_name: String,
    messages: Vec<(String, String, Vec<MessageData>)>,
}

impl Expect for MessagesPageDataContext {
    const EXPECT: &'static str =
        "MessagesPageDataContext should be provided, this should never fail.";
}

#[server]
pub async fn get_messages_page_data(
    session_id: SessionUuid,
    max_messages: usize,
) -> Result<MessagesPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = MessagesPageDataContext {
        user_name: user.data.name.clone(),
        messages: Default::default(), /*user
                                      .data
                                      .groups
                                      .iter()
                                      .flatten()
                                      .take(max_messages)
                                      .flat_map(|group_id| server.get_group(group_id).map(|group| group.data.clone()))
                                      .collect(),*/
    };
    Ok(data)
}

pub struct MessagesPage;

impl GuardedComponentWithResource for MessagesPage {
    type DataContext = MessagesPageDataContext;
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
    async fn fetch(_: Self::Source) -> Option<Result<Self::DataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_messages_page_data(session_id, 10).await)
    }

    fn with_session_and_resource() -> impl IntoView {
        let messages_page_data = use_context::<MessagesPageDataContext>().expect_context();
        view! {
            <h1> "Hi there " {messages_page_data.user_name} "!" </h1>
            <AdColumns>
                <h2> "Groups you are currently subscribed to or following: "</h2>
                <RoundedBox>
                    <For
                        each = move ||messages_page_data.messages.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,_message)| view!{
                            /*<LabelledControlStack label = {message.name} href = {Some(format!("/group/{}", group.id.to_string()))} class = "w-1/2">
                                <ButtonControl value = "Unsubscribe" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>*/
                        }
                    />
                </RoundedBox>
            </AdColumns>
        }
    }
}

impl GuardedComponentWithoutSession for MessagesPage {
    fn without_session() -> impl IntoView {
        view!{
            <LoginBox />
        }
    }
}

impl GuardedPage for MessagesPage {}