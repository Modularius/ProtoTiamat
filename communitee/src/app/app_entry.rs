use crate::{
    app::{
        components::{FootBar, SessionView, TopBar},
        pages::{FriendlistPage, GroupPage, GroupslistPage, HomePage, LoginPage, RegisterPage, UserPage},
    },
    server::require_login,
    structs::{ClientSideData, Session},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

/// This struct enable a degree of type-checking for the [use_context]/[use_context] functions.
/// Any component making use of the following fields should call `use_context::<TopLevelContext>()`
/// and select the desired field.
#[derive(Clone)]
pub struct TopLevelContext {
    pub client_side_data: ClientSideData,
    pub session: Resource<Result<Option<Session>, ServerFnError>>,
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client_side_data = SharedValue::new(|| {
        use_context::<ClientSideData>()
            .expect("TopLevelContext should be provided, this should never fail.")
    })
    .into_inner();

    #[cfg(feature = "hydrate")]
    let public_path = client_side_data.public_url.path().to_string();

    let session = Resource::new_blocking(|| (), move |_| require_login());

    provide_context(TopLevelContext {
        client_side_data,
        session,
    });

    view! {
        <SessionView
            fallback=move || view!{<TopBar user_data = None/>}
            action=|session| view!{ <TopBar user_data = Some(session.user_data.clone()) /> }
        />
        <Router base = "" /*base=cfg_if! { if #[cfg(feature = "hydrate")] { public_path } else { "" } }*/>
            <Routes fallback = NotFound>
                <Route path = path!("/") view = HomePage />
                <Route path = path!("/register") view = RegisterPage />
                <Route path = path!("/login") view = LoginPage />
                <Route path = path!("/friends") view = FriendlistPage />
                <Route path = path!("/groups") view = GroupslistPage />
                <Route path = path!("/user/:user_id") view = UserPage />
                <Route path = path!("/group/:group_id") view = GroupPage />
                <Route path = path!("/help") view = HomePage />
            </Routes>
        </Router>
        <FootBar />
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <p> "Communitee: URL not found" </p>
    }
}
