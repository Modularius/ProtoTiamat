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
    let session = use_context::<TopLevelContext>()
        .expect_context()
        .session_id;

    move || Show(ShowProps {
        children: children.clone(),
        when: move || session.get_untracked().is_some(),
        fallback: Default::default(),
    })
}

#[component]
pub fn NotLoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let session = use_context::<TopLevelContext>().expect_context().session_id;
    move || Show(ShowProps {
        children: children.clone(),
        when: move || session.get().is_none(),
        fallback: Default::default(),
    })
}
