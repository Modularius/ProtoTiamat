use crate::{
    app::pages::{
        FriendlistPage, GroupPage, GroupslistPage, HelpPage, HomePage, LoginPage, MessagesPage,
        RegisterPage, UserPage,
    },
    //server_functions::{PerformLogin, PerformLogout, get_session_from_identity},
    structs::{ClientSideData, ContextExt, Expect, SessionActions},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    SsrMode, components::{Outlet, ParentRoute, Route, Router, Routes}, path
};
use libertee::SessionUuid;
use tracing::{Span, instrument};

/// This struct enable a degree of type-checking for the [use_context]/[use_context] functions.
/// Any component making use of the following fields should call `use_context::<TopLevelContext>()`
/// and select the desired field.
#[derive(Clone)]
pub struct TopLevelContext<A: SessionActions> {
    pub client_side_data: ClientSideData,
    pub session_id_res: Resource<Result<Option<SessionUuid>, ServerFnError>>,
    pub session_id: Signal<Option<SessionUuid>>,
    pub login: ServerAction<A::Login>,
    pub logout: ServerAction<A::Logout>,
    pub span: Span,
}

impl<A: SessionActions> TopLevelContext<A> {
    #[inline]
    #[track_caller]
    pub fn session_id_expect(&self) -> SessionUuid {
        self.session_id_res
            .get()
            .unwrap()
            .expect("session_id_expect should only be called inside <SessionGuard>, this should never fail.")
            .expect("session_id_expect should only be called inside <IsLoggedIn>, this should never fail.")
    }
    
    #[inline]
    #[track_caller]
    pub fn login_expect(&self) -> SessionUuid {
        self.login
            .value()
            .get()
            .expect("session_id_expect should only be called inside <SessionGuard>, this should never fail.")
            .expect("session_id_expect should only be called inside <IsLoggedIn>, this should never fail.")
            
    }
}
/**/

impl<A: SessionActions> Expect for TopLevelContext<A> {
    const EXPECT: &'static str = "`TopLevelContext` should be provided, this should never fail.";
}

/// An app router which renders the homepage and handles 404's
#[component]
#[instrument(skip_all, parent=use_context::<Span>().and_then(|span|span.id()))]
pub fn App<A: SessionActions>(_a : A) -> impl IntoView {
    //provide_context(tracing::Span::current());
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client_side_data =
        SharedValue::new(|| use_context::<ClientSideData>().expect_context()).into_inner();

    //let public_path = client_side_data.public_url.router_base_form();

    let login = ServerAction::<A::Login>::new();
    let logout = ServerAction::<A::Logout>::new();
    let session_id_res: Resource<Result<Option<SessionUuid>, ServerFnError>> = Resource::new_blocking(
            move|| (login.version().get(), logout.version().get()),
            A::GET_SESSION_FROM_IDENTITY
    );
    let session_id = Signal::derive(move || session_id_res
        .get()
        .and_then(|session_id_res|
            match session_id_res {
                Ok(session_id_res) => session_id_res,
                Err(e) => {
                    tracing::error!("{e}");
                    None
                }
            }
        )
    );
    provide_context(TopLevelContext::<A> {
        client_side_data,
        session_id_res,
        session_id,
        login,
        logout,
        span: Span::current()
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
