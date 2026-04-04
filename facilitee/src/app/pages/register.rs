use crate::app::{
    components::{FootBar, MainColumn, RegisterBox, TopBar},
    generic_components::RoundedBox,
    guards::SessionGuard,
};
use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
                <MainColumn>
                    <h2>"Welcome To Communitee."</h2>
                    <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
                    <RoundedBox>
                        <RegisterBox />
                    </RoundedBox>
                </MainColumn>
            <FootBar/>
        </SessionGuard>
    }
}
