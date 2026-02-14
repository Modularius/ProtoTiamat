use leptos::prelude::*;
use leptos_router::components::A;
use libertee::Session;

use crate::app::{components::LoginBox, generic_components::{ButtonControl, ButtonFunction, ControlStack, LabelledControlStack, SessionView}};

#[component]
fn CommuniteeTitle() -> impl IntoView {
    view!{
        <div class = "text-4xl text-nowrap text-purple-100 text-center text-shadow-2xl w-1/2">
            <A href = "/"> "Communitee" </A>
        </div>
    }
}

#[component]
fn SanctimoneousMissionStatement() -> impl IntoView {
    view!{
        <div class = "text-lg  text-nowrap text-purple-100 text-center text-shadow-xl  w-1/4 hidden md:block">
            "The internet we were promised"
        </div>
    }
}

#[component]
fn RightBar(children: Children) -> impl IntoView {
    view!{
        <div class = "text-purple-100 w-1/2 md:w-1/4">
            {children()}
        </div>
    }
}

#[component]
fn BigBar(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-indigo-700 flex flex-col md:flex-row items-center justify-even">
            {children()}
        </div>
    }
}

#[component]
fn ToolBar(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-green-700 text-green-100 text-sm md:text-lg flex flex-row items-center p-1 gap-x-1 gap-y-1 justify-stretch md:justify-even">
            {children()}
        </div>
    }
}

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <BigBar>
            <SanctimoneousMissionStatement/>
            <CommuniteeTitle/>
            <RightBar>
                <SessionView
                    fallback = ||LoginBar()
                    action = |session: &Session|UserBar(UserBarProps { session: session.clone() })
                />
            </RightBar>
        </BigBar>
        <ToolBar>
            <ButtonControl value = "Your Feed" on_click = ButtonFunction::Link("/") />
            <ButtonControl value = "Your Friends" on_click = ButtonFunction::Link("/friends") />
            <ButtonControl value = "Your Groups" on_click = ButtonFunction::Link("/groups") />
            <ButtonControl value = "Your Posts" on_click = ButtonFunction::Link("/posts") />
            <ButtonControl value = "Favourites" on_click = ButtonFunction::Link("/favourites") />
            <ButtonControl value = "Help" on_click = ButtonFunction::Link("/help") />
        </ToolBar>
    }
}

#[component]
fn UserBar(session: Session) -> impl IntoView {
    view! {
        <LabelledControlStack label = {session.user_data.name} href = {Some(format!("/user/{}", session.user_data.id.to_string()))} class = "w-1/3">
            <ButtonControl value = "Settings" on_click = ButtonFunction::closure(|_ev|{}) />
            <ButtonControl value = "Logout" on_click = ButtonFunction::closure(|_ev|{})/>
        </LabelledControlStack>
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