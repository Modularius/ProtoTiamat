use leptos::{either::Either, prelude::*};
use leptos_router::components::A;

use crate::{app::{components::LoginBox, generic_components::{ButtonControl, ControlStack, SessionView}}, structs::Session};

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class = "bg-indigo-700 flex flex-col md:flex-row justify-even">
            <div class = "text-lg text-nowrap text-purple-100 align-bottom w-1/4"> "The internet we were promised" </div>
            <div class = "text-4xl text-nowrap text-purple-100 text-center text-shadow-lg w-1/2"> "Communitee" </div>
            <div class = "text-purple-100 w-1/4">
                <SessionView
                    fallback = ||LoginBar()
                    action = |session|UserBar(UserBarProps { session: session.clone() })
                />
            </div>
        </div>
    }
}

#[component]
pub fn UserBar(session: Session) -> impl IntoView {
    view! {
        <ControlStack>
            <div class = "text-nowrap m-1 p-1 w-full">
                <A href = format!("/user/{}", session.user_data.id)>{session.user_data.name}</A>
            </div>
            <ButtonControl value = "Settings" on_click = |_ev|{} />
            <ButtonControl value = "Logout" on_click = |_ev|{}/>
        </ControlStack>
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
