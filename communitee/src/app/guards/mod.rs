mod resource_guard;
mod session_guard;
//mod user_guard;

pub use resource_guard::{PageGuard, ResourceGuard};
pub use session_guard::SessionGuard;

use leptos::prelude::*;

use crate::{app::TopLevelContext, structs::ContextExt};
//use libertee::{Session, SessionUuid, UserData};

#[component]
pub fn IsLoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let session = use_context::<TopLevelContext>().expect_context().login.value();
    Show(ShowProps {
        children: children.clone(),
        when: move || session.get().is_some_and(|res|res.is_ok()),
        fallback: Default::default(),
    })
}

#[component]
pub fn NotLoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let session = use_context::<TopLevelContext>().expect_context().login.value();
    Show(ShowProps {
        children: children.clone(),
        when: move || session.get().is_none_or(|res|res.is_err()),
        fallback: Default::default(),
    })
}
