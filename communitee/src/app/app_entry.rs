use crate::{
    app::{components::{FootBar, TopBar}, generic_components::LoggedInGuard, pages::{
            FriendlistPage, GroupPage, GroupslistPage, HomePage, LoginPage, RegisterPage, UserPage,
        }
    },
    server_functions::{get_session_from_identity, require_login},
    structs::{ClientSideData, Expect},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use libertee::{Session, SessionUuid, UserUuid};

/// This struct enable a degree of type-checking for the [use_context]/[use_context] functions.
/// Any component making use of the following fields should call `use_context::<TopLevelContext>()`
/// and select the desired field.
#[derive(Clone)]
pub struct TopLevelContext {
    pub client_side_data: ClientSideData,
    pub session: Resource<Result<Option<SessionUuid>, ServerFnError>>,
    pub session_id: RwSignal<Option<SessionUuid>>,
    pub user_id: RwSignal<Option<UserUuid>>,
}

impl Expect for TopLevelContext {
    const EXPECT : &'static str = "`TopLevelContext` should be provided, this should never fail.";
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client_side_data = SharedValue::new(|| {
        use_context::<ClientSideData>()
            .expect("ClientSideData should be provided, this should never fail.")
    })
    .into_inner();

    let public_path = client_side_data.public_url.router_base_form();

    provide_context(TopLevelContext {
        client_side_data,
        session: Resource::new_blocking(|| (), move |_| get_session_from_identity()),
        session_id: RwSignal::new(None),
        user_id: RwSignal::new(None)
    });

    view! {
        <Router> //  base = public_path
            <LoggedInGuard>
                <TopBar/>
                <Routes fallback = NotFound>
                    <Route path = path!("/") view = HomePage />
                    <Route path = path!("/register") view = RegisterPage />
                    <Route path = path!("/login") view = LoginPage />
                    <Route path = path!("/friends") view = FriendlistPage />
                    <Route path = path!("/groups") view = GroupslistPage />
                    <ParentRoute path = path!("/user") view = ||view!{<Outlet />}>
                        <Route path = path!(":user_id") view = UserPage />
                    </ParentRoute>
                    <ParentRoute path = path!("/group") view = ||view!{<Outlet />}>
                        <Route path = path!(":group_id") view = GroupPage />
                    </ParentRoute>
                    <Route path = path!("/help") view = HomePage />
                </Routes>
                <FootBar />
            </LoggedInGuard>
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
