use crate::{
    app::components::{AccessBar, AdColumns, MainColumn, ResourceView, SessionView},
    server::get_user
};
use leptos::{Params, either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct UserParams {
    user_id: Option<String>,
}

#[component]
pub fn UserPage() -> impl IntoView {
    let params = use_params::<UserParams>();
    view! {
        <SessionView fallback=move || view! {} action = move |session| {
            let session = session.clone();
            let user_id = params.read()
                .as_ref()
                .ok()
                .and_then(move |p|p.user_id.clone())
                .unwrap_or_default();

            let user_data = Resource::new_blocking(||(), move|_| get_user(user_id.clone()));
            view!{
                <ResourceView
                    resource = user_data
                    action = move |user_data| {
                        let session = session.clone();
                        match user_data {
                            Some(user_data) => Either::Left(view!{
                                <MainColumn>
                                    <h1> "Hi there " {session.user_data.name.clone()} "!" </h1>
                                    <AccessBar user_data = user_data.clone()/>
                                    <AdColumns>
                                        <h2> "Communitee User: " {user_data.name} </h2>
                                    </AdColumns>
                                </MainColumn>
                            }),
                            None => Either::Right(view!{
                                Missing
                            }),
                        }
                    }
                />
            }
        } />
    }
}