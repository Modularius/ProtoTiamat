use crate::{
    app::components::{AdColumns, MainColumn, ResourceView, SessionView},
    server_functions::get_user_page_data, structs::UserPageData
};
use leptos::{Params, either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

#[derive(Clone, Params, PartialEq)]
struct UserParams {
    user_id: Option<String>,
}

#[component]
pub fn UserPage() -> impl IntoView {
    || view! {
        <SessionView action = |session| {
            let session = session.clone();
            let params = use_params::<UserParams>();
            let user_id = move || params.get()
                .ok()
                .and_then(|params|params.user_id);
            
            let user_page_data = Resource::new_blocking(
                user_id,
                |user_id| get_user_page_data(user_id.clone())
            );
            view!{
                <MainColumn>
                    <h1> "Hi there " {session.user_data.name.clone()} "!" </h1>
                    //<AccessBar user_data = user_data.clone()/>
                    <AdColumns>
                        <div>
                        <ResourceView resource = user_page_data action = move |user_page_data| {
                            match user_page_data {
                                Some(user_page_data) => Either::Left(view!{
                                    <UserPageWithData user_page_data />
                                }),
                                None => Either::Right(view!{}),
                            }
                        }/>
                        </div>
                    </AdColumns>
                </MainColumn>
            }
        } />
    }
}

#[component]
pub fn UserPageWithData(user_page_data: UserPageData) -> impl IntoView {
    view!{
        <h2> "Communitee User: " {user_page_data.name} </h2>
        <h3> "Joined Communitee on: " {user_page_data.datetime_joined} </h3>
        <h3> "Has " {user_page_data.friends.len()} " friend(s)." </h3>
        <h3> "Is subscribed to " {user_page_data.groups_in.len()} " group(s)." </h3>
    }
}