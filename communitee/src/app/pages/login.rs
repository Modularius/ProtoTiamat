use crate::app::{components::{LoginBox, LogoutBox, MainColumn}, generic_components::{IsLoggedIn, NotLoggedIn}};
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <MainColumn>
            <h2>"Welcome To Communitee."</h2>
            <IsLoggedIn>
                <LogoutBox />
            </IsLoggedIn>
            <NotLoggedIn>
                <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
                <LoginBox />
            </NotLoggedIn>
        </MainColumn>
    }
}
