mod resource_guard;
mod session_guard;
//mod user_guard;

pub use resource_guard::{PageGuard, ResourceGuard};
pub use session_guard::SessionGuard;

use leptos::prelude::*;
use tracing::instrument;

use crate::{app::TopLevelContext, structs::ContextExt};
//use libertee::{Session, SessionUuid, UserData};

#[component]
#[instrument(skip_all)]
pub fn IsLoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let current_span = tracing::Span::current();
    move || {
        let session = use_context::<TopLevelContext>()
            .expect_context()
            .session_id_res
            .get()
            .and_then(|session_id_res| match session_id_res {
                Ok(session_id_res) => session_id_res,
                Err(e) => {
                    tracing::error!("{e}");
                    None
                }
            });

        current_span.in_scope(|| {
            Show(ShowProps {
                children: children.clone(),
                when: move || session.clone().is_some(),
                fallback: Default::default(),
            })
        })
    }
}

#[component]
#[instrument(skip_all)]
pub fn NotLoggedIn<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let current_span = tracing::Span::current();
    current_span.in_scope(|| {
        Show(ShowProps {
            children: children.clone(),
            when: move || use_context::<TopLevelContext>()
                .expect_context()
                .session_id_res
                .get()
                .and_then(|session_id_res| match session_id_res {
                    Ok(session_id_res) => session_id_res,
                    Err(e) => {
                        tracing::error!("{e}");
                        None
                    }
                }).is_none(),
            fallback: Default::default(),
        })
    })
}
