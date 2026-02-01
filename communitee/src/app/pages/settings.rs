use crate::app::components::{LoginBox, MainColumn};
use leptos::prelude::*;


#[component]
pub fn SettingsPage() -> impl IntoView {
    view!{
        <MainColumn>
            <h2>"Welcome To Communitee."</h2>
            <h3>"Please login to continue, or "<a href = "/register">"register"</a>" an account."</h3>
            <LoginBox />
        </MainColumn>
    }
}
