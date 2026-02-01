use leptos::prelude::*;

use crate::{app::components::generics::BoundLabelledInput, structs::UserData};

#[component]
pub fn UserSettings(user_data: UserData) -> impl IntoView {
    let user_name = RwSignal::<String>::new(user_data.name.clone());
    let user_email = RwSignal::<String>::new(user_data.name.clone());
    view!{
        <div class = "user-settings access-sub-bar">
            <div class = "user-settings access-sub-bar-inner">
                <div class = "section">
                    <BoundLabelledInput name = "user-name" label = "User Name" typ = "text" signal = user_name />
                    <BoundLabelledInput name = "user-email" label = "Email" typ = "text" signal = user_email />
                </div>
                <div class = "section">
                    <div class = "button"> "Close" </div>
                </div>
            </div>
        </div>
    }
}