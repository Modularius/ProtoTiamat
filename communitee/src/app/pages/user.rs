use crate::{
    app::components::{AccessBar, AdColumns, MainColumn, ResourceView, SessionView},
    server::get_user, structs::UserData
};
use leptos::{Params, either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct UserParams {
    user_id: Option<String>,
}

#[component]
pub fn UserPage() -> impl IntoView {
    view! {
        <SessionView fallback=move || view! {} action = move |session| {
            let session = session.clone();
            view!{ <UserPageWithUser user_data = session.user_data /> }
        } />
    }
}

#[component]
pub fn UserPageWithUser(user_data: UserData) -> impl IntoView {
    move || {
        let user_data = user_data.clone();
        let params = use_params::<UserParams>();
        let user_id = params.read()
            .as_ref()
            .ok()
            .and_then(move |p|p.user_id.clone())
            .unwrap_or_default();

        let subject_user_data = Resource::new_blocking(||(), move|_| get_user(user_id.clone()));
        view!{
            <MainColumn>
                <h1> "Hi there " {user_data.name.clone()} "!" </h1>
                //<AccessBar user_data = user_data.clone()/>
                <AdColumns>
                    <div></div>
                    <ResourceView
                        resource = subject_user_data
                        action = move |subject_user_data|
                            view!{<UserPageWithUserAndSubject subject_user_data = subject_user_data />}
                    />
                </AdColumns>
            </MainColumn>
        }
    }
}

#[component]
fn UserPageWithUserAndSubject(subject_user_data: Option<UserData>) -> impl IntoView {
    match subject_user_data {
        Some(subject_user_data) => Either::Left(view! {
            <h2> "Communitee User: " {subject_user_data.name} </h2>
        }),
        None => Either::Right(view! {
            Missing User
        })
    }
}