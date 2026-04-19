use abilitee::app::{
    components::{AdColumns, FootBar, LogoutBox, MainColumn, RegisterBox, TopBar},
    generic_components::RoundedBox, guards::{GuardedComponent, GuardedComponentWithoutSession, GuardedPage},
};
use leptos::prelude::*;
use tracing::instrument;

pub struct RegisterPage;

impl GuardedComponent for RegisterPage {
    #[instrument]
    fn with_session() -> impl IntoView {
        view! {
            <AdColumns>
                <LogoutBox />
            </AdColumns>
        }
    }
}

impl GuardedComponentWithoutSession for RegisterPage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            <h2>"Welcome To Communitee."</h2>
            <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
            <RoundedBox>
                <RegisterBox />
            </RoundedBox>
        }
    }
}

impl GuardedPage for RegisterPage {}
