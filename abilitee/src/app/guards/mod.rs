mod resource_guard;
mod session_guard;
//mod user_guard;

pub use resource_guard::{PageGuard, ResourceGuard};
use serde::{Deserialize, Serialize};
pub use session_guard::SessionGuard;

use leptos::{either::Either, prelude::*};
use tracing::instrument;

use crate::{Expect, app::{TopLevelContext, components::{FootBar, MainColumn}, generic_components::error_box}, structs::ContextExt};
//use libertee::{Session, SessionUuid, UserData};

pub trait GuardedPage: Send + Sync + 'static {
    type DataContext : Clone + Serialize + for<'a>Deserialize<'a> + Expect + Send + Sync + 'static;
    type Source: PartialEq + Clone + Send + Sync + 'static;

    fn source() -> Self::Source;
    fn fetch(_: Self::Source) -> impl Future<Output = Option<Result<Self::DataContext, ServerFnError>>> + Send;
    fn with_data() -> impl IntoView;
    fn without_session() -> impl IntoView;
    
    fn with_session() -> impl IntoView {
        let resource = Resource::new(Self::source, Self::fetch);
        view! {
            <Transition>
                {move || resource.get()
                    .flatten()
                    .map(|value| view! {
                        <ErrorBoundary fallback = error_box>
                            {value.map(|value| {
                                provide_context(value);
                                Self::with_data
                            })}
                        </ErrorBoundary>
                    })
                }
            </Transition>
        }
    }

    fn component() -> impl IntoView {
        view! {
            <MainColumn>
                <Suspense>
                    {move || {
                        let top_level_context = use_context::<TopLevelContext>()
                            .expect_context();
                        let session_id = top_level_context
                            .session_id
                            .get()
                            .and_then(|session_id|session_id
                                .inspect_err(|e|tracing::error!("{e}"))
                                .ok()
                            ).flatten();
                        if session_id.is_some() {
                            Either::Left(Self::with_session)
                        } else {
                            Either::Right(Self::without_session)
                        }

                    }}
                </Suspense>
            </MainColumn>
            <FootBar />
        }
    }
}

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
            .session_id
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
                .session_id
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
