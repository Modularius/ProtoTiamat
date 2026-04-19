use abilitee::app::{
    components::{AdColumns, LoginBox, LogoutBox}, guards::{GuardedComponent, GuardedComponentWithoutSession, GuardedPage},
};
use leptos::prelude::*;
use tracing::instrument;

pub struct LoginPage;

impl GuardedComponent for LoginPage {
    #[instrument]
    fn with_session() -> impl IntoView {
        view! {
            <AdColumns>
                <LogoutBox />
            </AdColumns>
        }
    }
}

impl GuardedComponentWithoutSession for LoginPage {
    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            "Not Logged In"
            <LoginBox />
        }
    }
}

impl GuardedPage for LoginPage {}
