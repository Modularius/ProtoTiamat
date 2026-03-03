use crate::{
    app::{TopLevelContext, generic_components::error_box},
    structs::ContextExt,
};
use leptos::prelude::*;
use tracing::info_span;

#[component]
pub fn SessionGuard<C>(children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
{
    let top_level_context = use_context::<TopLevelContext>().expect_context();
    let session = top_level_context.session;

    move || {
        let _guard = info_span!("SessionGuard").entered();
        let children = children.clone();
        let session = session.clone();
        Suspend::new(async move {
            let span = info_span!("SessionGuard Suspense");
            let _guard = span.enter();
            let session_id = session.await;
            view! {
                <ErrorBoundary fallback = error_box>
                    {
                        session_id.map(|session_id| {
                            top_level_context.session_id.set(session_id);
                            children.into_inner()()
                        })
                    }
                </ErrorBoundary>
            }
        })
    }
}
