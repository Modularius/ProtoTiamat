use crate::{app::components::LabelledInput, server::PerformLogin};
use leptos::prelude::*;

#[component]
pub fn LoginBox() -> impl IntoView {
    let login = ServerAction::<PerformLogin>::new();
    view!{
        <div class = "login-box">
            <ActionForm action = login>
                <div class = "login-box-inner">
                        <LabelledInput name = "username" label = "Username" typ = "text" value = ""/>
                        <LabelledInput name = "password" label = "password" typ = "password" value = ""/>
                        <input type = "submit" value = "Login" />
                </div>
            </ActionForm>
        </div>
    }
}