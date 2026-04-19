use abilitee::{ContextExt, Expect, TopLevelContext, app::{
    components::{AdColumns, LoginBox}, generic_components::RoundedBox, guards::{GuardedComponentWithResource, GuardedComponentWithoutSession, GuardedPage}
}};
use leptos::prelude::*;
use libertee::SessionUuid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use abilitee::ServerSideData;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsPageDataContext {
    user_name: String,
}

impl Expect for SettingsPageDataContext {
    const EXPECT: &'static str =
        "SettingsPageDataContext should be provided, this should never fail.";
}

#[server]
pub async fn get_settings_page_data(session_id: SessionUuid) -> Result<SettingsPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = SettingsPageDataContext {
        user_name: user.data.name.clone(),
    };
    Ok(data)
}

pub struct SettingsPage;

impl GuardedComponentWithResource for SettingsPage {
    type DataContext = SettingsPageDataContext;
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
        Some(get_settings_page_data(session_id).await)
    }

    fn with_session_and_resource() -> impl IntoView {
        let settings_page_data = use_context::<SettingsPageDataContext>().expect_context();
        view! {
            <h1> "Hi there " {settings_page_data.user_name} "!" </h1>
            <AdColumns>
                <h2> "Groups you are currently subscribed to or following: "</h2>
                <RoundedBox>
                Nowt
                </RoundedBox>
            </AdColumns>
        }
    }

}

impl GuardedComponentWithoutSession for SettingsPage {
    fn without_session() -> impl IntoView {
        view!{
            <LoginBox />
        }
    }
}

impl GuardedPage for SettingsPage {}