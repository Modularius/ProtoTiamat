use tracing::instrument;
use abilitee::{
    ContextExt, Expect, app::{
        TopLevelContext,
        components::{
            AdColumns, HelpBox, LoginBox
        }, guards::{GuardedComponentWithoutSession, GuardedComponentWithResource, GuardedPage},
    }
};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use abilitee::ServerSideData;
} }

#[derive(Clone, Serialize, Deserialize)]
pub struct HelpPageDataContext {
    user_id: UserUuid,
    user_name: String,
}

impl Expect for HelpPageDataContext {
    const EXPECT: &'static str = "HelpPageDataContext should be provided, this should never fail.";
}

#[server]
#[instrument]
pub async fn get_help_page_data(
    session_id: SessionUuid,
    max_posts: usize,
) -> Result<HelpPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server.get_session(&session_id)?;

    let user = server.get_user(&session.user)?;

    let data = HelpPageDataContext {
        user_id: user.data.id.clone(),
        user_name: user.data.name.clone(),
    };
    Ok(data)
}

pub struct HelpPage;

impl GuardedComponentWithResource for HelpPage {
    type DataContext = HelpPageDataContext;
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
    async fn fetch(_: Self::Source) -> Option<Result<HelpPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_help_page_data(session_id, 10).await)
    }

    #[instrument]
    fn with_session_and_resource() -> impl IntoView {
        let home_page_data = use_context::<HelpPageDataContext>().expect_context();
        view! {
            <h1 class = "text-3xl m-6"> "Hi there " {home_page_data.user_name.clone()} "!" </h1>
            <AdColumns>
                <HelpBox />
            </AdColumns>
        }
    }

}

impl GuardedComponentWithoutSession for HelpPage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
            <HelpBox />
            <LoginBox />
        }
    }
}

impl GuardedPage for HelpPage {}
