use crate::{
    app::{
        components::{AdColumns, FootBar, MainColumn, TopBar},
        generic_components::RoundedBox,
        guards::{IsLoggedIn, NotLoggedIn, PageGuard, SessionGuard},
    },
    structs::{ContextExt, Expect},
};
use leptos::prelude::*;
use libertee::{LiberteeError, SessionUuid};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

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
        .get_session(&session_id)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    let user = server
        .get_user(&session.user)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

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

#[component]
pub fn MessagesPage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
                <IsLoggedIn>
                    <PageGuard with_parameters = |session_id|GetMessagesPageData{ session_id, max_messages: 10 }>
                        <MessagesPageWithData />
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
pub fn MessagesPageWithData() -> impl IntoView {
    let messages_page_data = use_context::<MessagesPageDataContext>().expect_context();
    view! {
        <MainColumn>
            <h1> "Hi there " {messages_page_data.user_name} "!" </h1>
            <AdColumns>
                <h2> "Groups you are currently subscribed to or following: "</h2>
                <RoundedBox>
                    <For
                        each = move ||messages_page_data.messages.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = |(_,message)| view!{
                            /*<LabelledControlStack label = {message.name} href = {Some(format!("/group/{}", group.id.to_string()))} class = "w-1/2">
                                <ButtonControl value = "Unsubscribe" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>*/
                        }
                    />
                </RoundedBox>
            </AdColumns>
        </MainColumn>
    }
}
