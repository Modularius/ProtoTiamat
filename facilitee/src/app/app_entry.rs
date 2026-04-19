use crate::app::pages::{
    FavouritesPage, FriendslistPage, GroupPage, GroupslistPage, HelpPage, HomePage, LoginPage, MessagesPage, RegisterPage, SettingsPage, UserPage
};
use abilitee::{
    ClientSideData, ContextExt, PerformLogin, PerformLogout, TopLevelContext, app::guards::GuardedPage, get_session_from_identity
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    SsrMode,
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use libertee::SessionUuid;
use tracing::{Span, instrument};

/// An app router which renders the homepage and handles 404's
#[component]
#[instrument(skip_all, parent=use_context::<Span>().and_then(|span|span.id()))]
pub fn App() -> impl IntoView {
    //provide_context(tracing::Span::current());
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let client_side_data =
        SharedValue::new(|| use_context::<ClientSideData>().expect_context()).into_inner();

    //let public_path = client_side_data.public_url.router_base_form();

    let login = ServerAction::<PerformLogin>::new();
    let logout = ServerAction::<PerformLogout>::new();

    let session_id: Resource<Result<Option<SessionUuid>, ServerFnError>> =
        Resource::new_blocking(
            move || (login.version().get(), logout.version().get()),
            |_| get_session_from_identity(),
        );
    provide_context(TopLevelContext {
        client_side_data,
        session_id,
        login,
        logout,
        span: Span::current(),
    });

    view! {
        <Router> // base = public_path>
            <Routes fallback = NotFound>
                <Route path = path!("/") view = HomePage::page ssr = SsrMode::Async />
                <Route path = path!("/register") view = RegisterPage::page ssr = SsrMode::Async />
                <Route path = path!("/favourites") view = FavouritesPage::page ssr = SsrMode::Async />
                <Route path = path!("/login") view = LoginPage::page ssr = SsrMode::Async />
                <Route path = path!("/friends") view = FriendslistPage::page ssr = SsrMode::Async />
                <Route path = path!("/groups") view = GroupslistPage::page ssr = SsrMode::Async />
                <Route path = path!("/settings") view = SettingsPage::page ssr = SsrMode::Async />
                <Route path = path!("/messages") view = MessagesPage::page ssr = SsrMode::Async />
                <ParentRoute path = path!("/user") view = ||view!{<Outlet />}>
                    <Route path = path!(":user_id") view = UserPage::page ssr = SsrMode::Async />
                </ParentRoute>
                <ParentRoute path = path!("/group") view = ||view!{<Outlet />}>
                    <Route path = path!(":group_id") view = GroupPage::page ssr = SsrMode::Async />
                </ParentRoute>
                <Route path = path!("/help") view = HelpPage::page ssr = SsrMode::Async />
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
