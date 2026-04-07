use leptos::{prelude::*, server_fn::ServerFn};
use tracing::{Span, info, info_span, instrument};

use crate::{
    app::{TopLevelContext, generic_components::error_box},
    structs::ContextExt,
};
use libertee::SessionUuid;

#[component]
#[instrument(skip_all)]
pub fn ResourceGuard<T, C>(
    resource: Resource<Option<Result<T, ServerFnError>>>,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    C: IntoView + 'static,
{
    let children = StoredValue::new(children);
    info!("Running Resource Guard.");
    /*let session_id = use_context::<TopLevelContext>()
    .expect_context()
    .session_id_expect();*/
    move || {
        Span::current().in_scope(|| {
            //let children = children.clone();
            view! {
                <Transition>
                {
                    move || Span::current().in_scope(||{
                        let children = children.clone();
                        resource.get()
                            .flatten()
                            .map(|value| {
                                view! {
                                    <ErrorBoundary fallback = error_box>
                                    {
                                        value.map(|value| {
                                            provide_context(value);
                                            Span::current()
                                                .in_scope(||children
                                                    .with_value(|children|children.clone().into_inner()())
                                                )
                                        })
                                    }
                                    </ErrorBoundary>
                                }
                            })
                    })
                }
                </Transition>
            }
        })
    }
    /*
    let children = children.clone();
    let future = async move |parent_span| {
        let span = info_span!(parent: &parent_span, "PageGuard Suspense");
        let _guard = span.enter();

        info!("A Little Info.");
        resource.get()
            .flatten()
            .map(|value| {
                view! {
                    <ErrorBoundary fallback = error_box>
                    {
                        value.map(|value| {
                            provide_context(value);
                            children.into_inner()()
                        })
                    }
                    </ErrorBoundary>
                }
            })
    };
    Suspend::new(future(span.clone()))
    */
}

#[component]
#[instrument(skip_all)]
pub fn PageGuard<S, P, C>(with_parameters: P, children: TypedChildrenFn<C>) -> impl IntoView
where
    C: IntoView + 'static,
    Result<View<C>, <S as ServerFn>::Error>: IntoView,
    P: Fn(SessionUuid) -> S + Send + 'static,
    S: ServerFn + Clone + Send + Sync + 'static,
    <S as ServerFn>::Output: Clone + Sync + 'static,
    <S as ServerFn>::Error: Clone,
{
    let server_action = ServerAction::<S>::new();
    move || {
        let span = info_span!("PageGuard");
        let _guard = span.enter();
        let session_id = use_context::<TopLevelContext>()
            .expect_context()
            .session_id_expect();

        server_action.dispatch(with_parameters(session_id));

        let children = children.clone();
        let future = async move |parent_span| {
            let span = info_span!(parent: &parent_span, "PageGuard Suspense");
            let _guard = span.enter();

            info!("A Little Info.");
            server_action.value().get().map(|value| {
                view! {
                    <ErrorBoundary fallback = error_box>
                    {
                        value.map(|value| {
                            provide_context(value);
                            children.into_inner()()
                        })
                    }
                    </ErrorBoundary>
                }
            })
        };
        Suspend::new(future(span.clone()))
    }
}
