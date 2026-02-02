use leptos::{either::Either, prelude::*};

use crate::{app::components::LoginBox, structs::UserData};

#[component]
pub fn TopBar(user_data: Option<UserData>) -> impl IntoView {
    view! {
        <div class = "top-bar">
            <div class = "top-bar-inner">
                <div class = "tagline"> "The Internet we were Promised" </div>
                <div class = "title"> "Communitee" </div>
                <div class = "right-bar">
                    {if let Some(user_data) = user_data {
                        Either::Left(UserBar(UserBarProps { user_data }))
                    } else {
                        Either::Right(LoginBar())
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn UserBar(user_data: UserData) -> impl IntoView {
    view! {
        <div class = "user-stack">
            <div class = "user-name"> {user_data.name} </div>
            <div class = "button user-settings"> {} </div>
            <div class = "button user-logout"> {} </div>
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
