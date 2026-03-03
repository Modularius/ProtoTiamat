use crate::{
    app::pages::{
        FriendlistPage, GroupPage, GroupslistPage, HelpPage, HomePage, LoginPage, MessagesPage,
        RegisterPage, UserPage,
    },
    server_functions::get_session_from_identity,
    structs::{ClientSideData, ContextExt, Expect},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    SsrMode, components::{Outlet, ParentRoute, Route, Router, Routes}, path
};
use libertee::SessionUuid;

/// This struct enable a degree of type-checking for the [use_context]/[use_context] functions.
/// Any component making use of the following fields should call `use_context::<TopLevelContext>()`
/// and select the desired field.
#[derive(Clone)]
pub struct TopLevelContext {
    pub client_side_data: ClientSideData,
    pub session: Resource<Result<Option<SessionUuid>, ServerFnError>>,
    pub session_id: RwSignal<Option<SessionUuid>>,
}

impl TopLevelContext {
    pub fn session_id_expect(&self) -> SessionUuid {
        self.session_id
            .get()
            .expect("session_id should only be called inside <IsLoggedIn>, this should never fail.")
    }
}

impl Expect for TopLevelContext {
    const EXPECT: &'static str = "`TopLevelContext` should be provided, this should never fail.";
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client_side_data =
        SharedValue::new(|| use_context::<ClientSideData>().expect_context()).into_inner();

    //let public_path = client_side_data.public_url.router_base_form();

    provide_context(TopLevelContext {
        client_side_data,
        session: Resource::new_blocking(|| (), |_| {tracing::warn!("This fetcher is being called."); get_session_from_identity()}),
        session_id: RwSignal::new(None),
    });

    view! {
        <Router> // base = public_path>
            <Routes fallback = NotFound>
                <Route path = path!("/") view = HomePage ssr = SsrMode::Async />
                <Route path = path!("/register") view = RegisterPage ssr = SsrMode::Async />
                <Route path = path!("/login") view = LoginPage ssr = SsrMode::Async />
                <Route path = path!("/friends") view = FriendlistPage ssr = SsrMode::Async />
                <Route path = path!("/groups") view = GroupslistPage ssr = SsrMode::Async />
                <Route path = path!("/messages") view = MessagesPage ssr = SsrMode::Async />
                <ParentRoute path = path!("/user") view = ||view!{<Outlet />}>
                    <Route path = path!(":user_id") view = UserPage ssr = SsrMode::Async />
                </ParentRoute>
                <ParentRoute path = path!("/group") view = ||view!{<Outlet />}>
                    <Route path = path!(":group_id") view = GroupPage ssr = SsrMode::Async />
                </ParentRoute>
                <Route path = path!("/help") view = HelpPage ssr = SsrMode::Async />
        </Routes>
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    || {
        let loc = leptos_router::hooks::use_url().get();
        let origin = loc.origin().to_string();
        let path = loc.path().to_string();
        view! {
            <p> "Communitee: URL not found" </p>
            <p> "Communitee: " {origin} </p>
            <p> "Communitee: " {path} </p>
        }
    }
}
