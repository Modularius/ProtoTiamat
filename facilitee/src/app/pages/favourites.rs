use abilitee::{
    ContextExt, Expect, TopLevelContext, app::{
        components::{AdColumns, LoginBox},
        guards::{GuardedComponentWithResource, GuardedComponentWithoutSession, GuardedPage},
    }
};
use leptos::prelude::*;
use libertee::{SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FavouritesData {
    id: UserUuid,
    name: String,
    link: String,
}

#[cfg(feature = "ssr")]
impl FavouritesData {
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FavouritesDataPageDataContext {
    user_name: String,
}

impl Expect for FavouritesDataPageDataContext {
    const EXPECT: &'static str = "FavouritesDataPageDataContext should be provided, this should never fail.";
}

#[server]
async fn get_friendslist_page_data(
    session_id: SessionUuid,
    max_favourites: usize,
) -> Result<FavouritesDataPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = FavouritesDataPageDataContext {
        user_name: user.data.name.clone()
    };

    Ok(data)
}

pub struct FavouritesPage;

impl GuardedComponentWithResource for FavouritesPage {
    type DataContext = FavouritesDataPageDataContext;
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
    async fn fetch(_: Self::Source) -> Option<Result<FavouritesDataPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_friendslist_page_data(session_id, 10).await)
    }

    #[instrument]
    fn with_session_and_resource() -> impl IntoView {
        let friendslist_page_data = use_context::<FavouritesDataPageDataContext>().expect_context();
        view! {
            <h1 class = "text-3xl m-6"> "Hi there " {friendslist_page_data.user_name} "!" </h1>
            //<AccessBar user_data = user_data.clone()/>
            <AdColumns>
            Nowt
                /*<h2> {format!("You have {} favourite post(s)", friendslist_page_data.friends.len())} </h2>
                <SharpBox>
                    <For
                        each = move ||friendslist_page_data.friends.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = move |(_,friend)| view!{
                            <LabelledControlStack label = {friend.name} href = {Some(friend.link)} class = "w-1/2">
                                <ButtonControl value = "Delegate" on_click = ButtonFunction::closure(|_|{}) />
                                <ButtonControl value = "Unfriend" on_click = ButtonFunction::closure(|_|{}) />
                                <ButtonControl value = "Block" on_click = ButtonFunction::closure(|_|{}) />
                            </LabelledControlStack>
                        }
                    />
                </SharpBox>
                */
            </AdColumns>
        }
    }
}

impl GuardedComponentWithoutSession for FavouritesPage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            "Not Logged In"
            <LoginBox />
        }
    }
}

impl GuardedPage for FavouritesPage {}