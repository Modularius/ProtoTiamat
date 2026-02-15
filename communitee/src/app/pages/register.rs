use crate::app::{
    components::{MainColumn, RegisterBox},
    generic_components::RoundedBox,
};
use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <MainColumn>
            <h2>"Welcome To Communitee."</h2>
            <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
            <RoundedBox>
                <RegisterBox />
            </RoundedBox>
        </MainColumn>
    }
}
