use leptos::prelude::*;
use leptos_router::components::A;
use libertee::SessionUuid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    app::{
        TopLevelContext,
        generic_components::{ButtonControl, ButtonFunction, ControlStack, LabelledControlStack},
        guards::{IsLoggedIn, NotLoggedIn, ResourceGuard},
    },
    structs::{ContextExt, Expect},
};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
    use libertee::LiberteeError;
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
            <Suspense>
            <BigBar>
                <SanctimoneousMissionStatement/>
                <CommuniteeTitle/>
                <RightBar>
                    <IsLoggedIn>
                        {move || { view!{ <UserBar /> } } }
                    </IsLoggedIn>
                    <NotLoggedIn>
                        <LoginBar />
                    </NotLoggedIn>
                </RightBar>
            </BigBar>
            <ToolBar>
                <IsLoggedIn>
                    <ButtonControl value = "Your Feed" on_click = ButtonFunction::Link("/") />
                    <ButtonControl value = "Your Friends" on_click = ButtonFunction::Link("/friends") />
                    <ButtonControl value = "Your Groups" on_click = ButtonFunction::Link("/groups") />
                    <ButtonControl value = "Your Posts" on_click = ButtonFunction::Link("/posts") />
                    <ButtonControl value = "Favourites" on_click = ButtonFunction::Link("/favourites") />
                    <ButtonControl value = "Help" on_click = ButtonFunction::Link("/help") />
                </IsLoggedIn>
                <NotLoggedIn>
                    <ButtonControl value = "Home" on_click = ButtonFunction::Link("/") />
                    <ButtonControl value = "Login" on_click = ButtonFunction::Link("/login") />
                    <ButtonControl value = "Join Communitee" on_click = ButtonFunction::Link("/register") />
                    <ButtonControl value = "What is Communitee" on_click = ButtonFunction::Link("/help") />
                </NotLoggedIn>
            </ToolBar>
            </Suspense>
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserBarDataContext {
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
        .get_session(&session_id)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    let user = server
        .get_user(&session.user)
        .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;

    Ok(UserBarDataContext {
        user_name: user.data.name.clone(),
        user_page_href: format!("/user/{}", user.data.id.to_string()),
    })
}

#[component]
#[instrument]
fn UserBar() -> impl IntoView {
    let source = || {
        let top_level_context = use_context::<TopLevelContext>().expect_context();
        (
            top_level_context.login.version().get(),
            top_level_context.logout.version().get(),
        )
    };
    let fetch = async |_| {
        let session_id = use_context::<TopLevelContext>()
            .expect_context()
            .session_id_res
            .get()
            .and_then(|session_id_res| match session_id_res {
                Ok(session_id_res) => session_id_res,
                Err(e) => {
                    tracing::error!("{e}");
                    None
                }
            });
        if let Some(session_id) = session_id {
            Some(get_user_bar_data(session_id).await)
        } else {
            None
        }
    };
    view! {
        <Suspense>
        <ResourceGuard resource = Resource::new(source, fetch)>
        //<PageGuard with_parameters = |session_id|GetUserBarData{ session_id }>
        {
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
        //</PageGuard>
        </ResourceGuard>
        </Suspense>
    }
    /*
    move ||Suspend::new(async move {
        let user_bar_data = user_bar_action.value().get();
        user_bar_data.map(|user_bar_data|
            view! {
                <ErrorBoundary fallback = |_|{}> {
                    user_bar_data.map(|user_bar_data| {
                        let label = user_bar_data.user_name;
                        let href = Some(user_bar_data.user_page_href);
                        view!{
                            <LabelledControlStack label href class = "w-1/3">
                                <ButtonControl value = "Settings" on_click = ButtonFunction::closure(|_ev|{}) />
                                <ButtonControl value = "Logout" on_click = ButtonFunction::closure(|_ev|{})/>
                            </LabelledControlStack>
                        }
                    })
                }
                </ErrorBoundary>
            }
        )
    }) */
}

#[component]
#[instrument]
fn LoginBar() -> impl IntoView {
    view! {
        <ControlStack>
            <ButtonControl value = "Login" on_click = ButtonFunction::Link("/login") />
            <ButtonControl value = "Register" on_click = ButtonFunction::Link("/register") />
        </ControlStack>
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
