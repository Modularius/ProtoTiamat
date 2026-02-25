use crate::app::{components::{LoginBox, MainColumn}, generic_components::{IsLoggedIn, LoggedInGuard, NotLoggedIn}};
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <MainColumn>
            <h2>"Welcome To Communitee."</h2>
            <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
            <LoggedInGuard>
                <IsLoggedIn>
                    <div>"Log out"</div>
                </IsLoggedIn>
                <NotLoggedIn>
                    <LoginBox redirect_to = "/"/>
                </NotLoggedIn>
            </LoggedInGuard>
        </MainColumn>
    }
}
