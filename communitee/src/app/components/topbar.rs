use leptos::{either::Either, prelude::*};

use crate::{app::components::LoginBox, structs::Session};

#[component]
pub fn TopBar(session: Option<Session>) -> impl IntoView {
    view! {
        <div class = "bg-red-200 flex flex-row place-content-evenly align-center">
            <div class = "text-lg align-bottom"> "The internet we were promised" </div>
            <div class = "text-6xl"> "Communitee" </div>
            {if let Some(session) = session {
                Either::Left(UserBar(UserBarProps { session }))
            } else {
                Either::Right(LoginBar())
            }}
        </div>
    }
}

#[component]
pub fn UserBar(session: Session) -> impl IntoView {
    view! {
        <div class = "align-bottom flex flex-row justify-self-end">
            <div class = "text-nowrap">
                <a rel = "external" href = format!("/user/{}", session.user_data.id)>{session.user_data.name}</a>
            </div>
            <input type = "button" class = "w-8 bg-red-500 hover:bg-red-300" value = "S" alt = "Settings" />
            <input type = "button" class = "w-8 bg-red-500 hover:bg-red-300" value = "X" alt = "Logout"/>
        </div>
    }
}

#[component]
pub fn LoginBar() -> impl IntoView {
    view! {
        <div class = "login-stack">
            <LoginBox />
        </div>
    }
}
