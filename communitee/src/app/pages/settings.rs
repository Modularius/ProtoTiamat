use crate::app::{
    components::{FootBar, LoginBox, MainColumn, TopBar},
    guards::SessionGuard,
};
use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
                <MainColumn>
                    <h2>"Welcome To Communitee."</h2>
                    <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
                    <LoginBox />
                </MainColumn>
            <FootBar/>
        </SessionGuard>
    }
}
