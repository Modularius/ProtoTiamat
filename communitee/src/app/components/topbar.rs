use leptos::prelude::*;
use leptos_router::components::A;
use libertee::Session;

use crate::app::{
    components::LoginBox,
    generic_components::{
        ButtonControl, ButtonFunction, ControlStack, IsLoggedIn, LabelledControlStack, LoggedInContext, LoggedInGuard, NotLoggedIn, SessionView
    },
};

#[component]
fn CommuniteeTitle() -> impl IntoView {
    view! {
        <div class = "text-4xl text-nowrap text-purple-100 text-center text-shadow-2xl w-1/2">
            <A href = "/"> "Communitee" </A>
        </div>
    }
}

#[component]
fn SanctimoneousMissionStatement() -> impl IntoView {
    view! {
        <div class = "text-lg  text-nowrap text-purple-100 text-center text-shadow-xl  w-1/4 hidden md:block">
            "The internet we were promised"
        </div>
    }
}

#[component]
fn RightBar(children: Children) -> impl IntoView {
    view! {
        <div class = "text-purple-100 w-1/2 md:w-1/4">
            {children()}
        </div>
    }
}

#[component]
fn BigBar(children: Children) -> impl IntoView {
    view! {
        <div class = "bg-indigo-700 flex flex-col md:flex-row items-center justify-even">
            {children()}
        </div>
    }
}

#[component]
fn ToolBar(children: Children) -> impl IntoView {
    view! {
        <div class = "bg-green-700 text-green-100 text-sm md:text-lg flex flex-row items-center p-1 gap-x-1 gap-y-1 justify-stretch md:justify-even">
            {children()}
        </div>
    }
}

#[component]
pub fn TopBar() -> impl IntoView {
    || view! {
        <LoggedInGuard>
            <div></div>
        </LoggedInGuard>
        <LoggedInGuard>
            <BigBar>
                <SanctimoneousMissionStatement/>
                <CommuniteeTitle/>
                <RightBar>
                <IsLoggedIn>
                    <UserBar />
                </IsLoggedIn>
                <NotLoggedIn>
                    <LoginBar />
                </NotLoggedIn>
                </RightBar>
            </BigBar>
        </LoggedInGuard>
        <ToolBar>
            <LoggedInGuard>
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
            </LoggedInGuard>
        </ToolBar>
    }
}

#[component]
fn UserBar() -> impl IntoView {
    let session_id = use_context::<LoggedInContext>()
        .expect("LoggedInContext should exist, this should never fail.")
        .session_id;
    move || {
        let label = session_id.get()
            .expect("`UserBar` must only be used in `IsLoggedIn` block, this should never fail.")
            .to_string();

        //let label = "".to_string();//{session.user_data.name};
        let href = Some("".to_string());//{Some(format!("/user/{}", session.user_data.id.to_string()))};
        view! {
            <LabelledControlStack label href class = "w-1/3">
                <ButtonControl value = "Settings" on_click = ButtonFunction::closure(|_ev|{}) />
                <ButtonControl value = "Logout" on_click = ButtonFunction::closure(|_ev|{})/>
            </LabelledControlStack>
        }
    }
}

#[component]
fn LoginBar() -> impl IntoView {
    view! {
        <ControlStack>
            <ButtonControl value = "Login" on_click = ButtonFunction::Link("/login") />
            <ButtonControl value = "Register" on_click = ButtonFunction::Link("/register") />
        </ControlStack>
    }
}

#[component]
pub fn FootBar() -> impl IntoView {
    view! {
        <BigBar>
            <CommuniteeTitle/>
        </BigBar>
    }
}
