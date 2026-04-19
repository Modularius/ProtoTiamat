use leptos::{either::Either, prelude::*};
use leptos_router::components::A;
use libertee::SessionUuid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    app::{
        TopLevelContext,
        generic_components::{ButtonControl, ButtonFunction, ControlStack, LabelledControlStack},
        guards::{GuardedPage, has_session_id},
    },
    structs::{ContextExt, Expect},
};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[component]
#[instrument]
fn CommuniteeTitle() -> impl IntoView {
    view! {
        <div class = "text-4xl text-nowrap text-purple-100 text-center text-shadow-2xl w-1/2">
            <A href = "/"> "Communitee" </A>
        </div>
    }
}

#[component]
#[instrument]
fn SanctimoneousMissionStatement() -> impl IntoView {
    view! {
        <div class = "text-lg  text-nowrap text-purple-100 text-center text-shadow-xl  w-1/4 hidden md:block">
            "The internet we were promised"
        </div>
    }
}

#[component]
#[instrument(skip_all)]
fn RightBar(children: Children) -> impl IntoView {
    view! {
        <div class = "text-purple-100 w-1/2 md:w-1/4">
            {children()}
        </div>
    }
}

#[component]
#[instrument(skip_all)]
fn BigBar(children: Children) -> impl IntoView {
    view! {
        <div class = "bg-indigo-700 flex flex-col md:flex-row items-center justify-even">
            {children()}
        </div>
    }
}

#[component]
#[instrument(skip_all)]
fn ToolBar(children: Children) -> impl IntoView {
    view! {
        <div class = "bg-green-700 text-green-100 text-sm md:text-lg flex flex-row items-center p-1 gap-x-1 gap-y-1 justify-stretch md:justify-even">
            {children()}
        </div>
    }
}

#[component]
#[instrument]
pub fn TopBar() -> impl IntoView {
    {
        view! {
            <BigBar>
                <SanctimoneousMissionStatement/>
                <CommuniteeTitle/>
                <RightBar>
                    <Suspense> {
                        || if has_session_id() {
                            Either::Left(UserBar::component)
                        } else {
                            Either::Right(UserBar::without_session)
                        }
                    }</Suspense>
                </RightBar>
            </BigBar>
            <ToolBar>
                <Suspense> {
                    || if has_session_id() {
                        Either::Left(view!{
                            <ButtonControl value = "Your Feed" on_click = ButtonFunction::Link("/") />
                            <ButtonControl value = "Your Friends" on_click = ButtonFunction::Link("/friends") />
                            <ButtonControl value = "Your Groups" on_click = ButtonFunction::Link("/groups") />
                            <ButtonControl value = "Your Posts" on_click = ButtonFunction::Link("/posts") />
                            <ButtonControl value = "Favourites" on_click = ButtonFunction::Link("/favourites") />
                            <ButtonControl value = "Help" on_click = ButtonFunction::Link("/help") />
                        })
                    } else {
                        Either::Right(view!{
                            <ButtonControl value = "Home" on_click = ButtonFunction::Link("/") />
                            <ButtonControl value = "Login" on_click = ButtonFunction::Link("/login") />
                            <ButtonControl value = "Join Communitee" on_click = ButtonFunction::Link("/register") />
                            <ButtonControl value = "What is Communitee" on_click = ButtonFunction::Link("/help") />
                        })
                    }
                } </Suspense>
            </ToolBar>
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBarDataContext {
    user_name: String,
    user_page_href: String,
}

impl Expect for UserBarDataContext {
    const EXPECT: &'static str = "UserBarDataContext should be provided, this should never fail.";
}

#[server]
#[instrument]
async fn get_user_bar_data(session_id: SessionUuid) -> Result<UserBarDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    Ok(UserBarDataContext {
        user_name: user.data.name.clone(),
        user_page_href: format!("/user/{}", user.data.id.to_string()),
    })
}

pub struct UserBar;

impl GuardedPage for UserBar {
    type DataContext = UserBarDataContext;
    type Source = (usize,usize);

    fn source() -> Self::Source {
        let top_level_context = use_context::<TopLevelContext>().expect_context();
        (
            top_level_context.login.version().get(),
            top_level_context.logout.version().get(),
        )
    }

    async fn fetch(_: Self::Source) -> Option<Result<Self::DataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        Some(get_user_bar_data(session_id).await)
    }

    fn with_data() -> impl IntoView {
        let user_bar_data = use_context::<UserBarDataContext>().expect_context();
        let label = user_bar_data.user_name;
        let href = Some(user_bar_data.user_page_href);
        view!{
            <LabelledControlStack label href class = "w-1/3">
                <ButtonControl value = "Settings" on_click = ButtonFunction::closure(|_ev|{}) />
                <ButtonControl value = "Logout" on_click = ButtonFunction::closure(|_ev|{})/>
            </LabelledControlStack>
        }
    }

    fn without_session() -> impl IntoView {
        view! {
            <ControlStack>
                <ButtonControl value = "Login" on_click = ButtonFunction::Link("/login") />
                <ButtonControl value = "Register" on_click = ButtonFunction::Link("/register") />
            </ControlStack>
        }
    }
}

#[component]
#[instrument]
pub fn FootBar() -> impl IntoView {
    view! {
        <BigBar>
            <CommuniteeTitle/>
        </BigBar>
    }
}
