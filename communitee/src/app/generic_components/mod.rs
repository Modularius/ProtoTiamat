mod button;
mod control_stack;
mod error_box;
mod inert_containers;
mod labelled_input;

use std::sync::Arc;

pub use button::{ButtonControl, ButtonFunction, CloseButton, SubmitControl};
pub use control_stack::{Control, ControlStack, LabelledControlStack};
pub use error_box::error_box;
pub use inert_containers::{ErrorBox, RoundedBox, SharpBox};
pub use labelled_input::{LabelledInput, LabelledSelect, LabelledTextArea};
use leptos::{either::Either, prelude::*};

use crate::{app::{TopLevelContext, generic_components::inert_containers::ErrorBoxProps}, server_functions::get_session_from_identity};
use libertee::{Session, UserData};

#[component]
pub fn ResourceView<F, R, V>(
    resource: Resource<Result<R, ServerFnError>>,
    action: F,
    #[prop(optional, into)] fallback: ViewFnOnce,
) -> impl IntoView
where
    F: Fn(R) -> V + Sync + Send + 'static + Clone,
    V: IntoView + 'static,
    R: Clone + Sync + Send + 'static,
{
    view! {
        <Suspense fallback> {
            move || {
                let action = action.clone();
                resource.get().map(|resource| view!{
                    <ErrorBoundary fallback = error_box>
                        {resource.map(|resource|action.clone()(resource))}
                    </ErrorBoundary>
                })
            }
        }
        </Suspense>
    }
}

/// New-type wrapper for a function that returns a view with `From` and `Default` traits implemented
/// to enable optional props in for example `<Show>` and `<Suspense>`.
#[derive(Clone)]
pub struct ViewSessionFn(Arc<dyn Fn(Session) -> AnyView + Send + Sync + 'static>);

impl<F, C> From<F> for ViewSessionFn
where
    F: Fn(Session) -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(value: F) -> Self {
        Self(Arc::new(move |x| value(x).into_any()))
    }
}

impl ViewSessionFn {
    /// Execute the wrapped function
    pub fn run(&self, x : Session) -> AnyView {
        (self.0)(x)
    }
}

#[derive(Clone)]
pub struct LoggedInContext {
    pub session: Signal<Option<Session>>,
}

#[component]
pub fn LoggedInGuard<C>(
    children: TypedChildrenFn<C>,
) -> impl IntoView where C : IntoView + 'static
{
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;
    Suspend::new(async move { match session.await {
        Ok(session) => {
            provide_context(LoggedInContext { session: Signal::stored(session) });
            Either::Left(children.into_inner()())
        },
        Err(e) => {
            Either::Right(format!("{e:?}").into_any())
        },
    } })
}

#[component]
pub fn IsLoggedIn<C>(
    children: TypedChildrenFn<C>,
) -> impl IntoView where C : IntoView + 'static
{
    let session = use_context::<LoggedInContext>()
        .expect("LoggedInContext should exist, this should never fail.")
        .session;
    move ||Show(ShowProps { children: children.clone(), when: move ||session.get().is_some(), fallback: Default::default() })
}

#[component]
pub fn NotLoggedIn<C>(
    children: TypedChildrenFn<C>,
) -> impl IntoView where C : IntoView + 'static
{
    let session = use_context::<LoggedInContext>()
        .expect("LoggedInContext should exist, this should never fail.")
        .session;
    move ||Show(ShowProps { children: children.clone(), when: move ||session.get().is_none(), fallback: Default::default() })
}

#[component]
pub fn SessionView(
    #[prop(into)]
    action: ViewSessionFn
) -> impl IntoView {
    let session = use_context::<LoggedInContext>()
        .expect("LoggedInContext should exist, this should never fail.")
        .session
        .get()
        .expect("Session View");
    action.run(session)
}

/*#[component]
pub fn SessionView(
    #[prop(into)]
    action: ViewSessionFn,
    #[prop(optional, into)]
    fallback: ViewFn
) -> impl IntoView
{
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;
    view! {
        <Suspense>
            {move || session.get().map(|session| {
                let fallback = fallback.clone();
                let action = action.clone();
                view! {
                    <ErrorBoundary fallback = error_box>
                        {session.map(|session|session
                            .map(|session| action.run(session))
                            .unwrap_or_else(||fallback.run())
                        )}
                    </ErrorBoundary>
                }
            })}
        </Suspense>
        //<ResourceView resource = session action = move |session|session.as_ref().map(action.clone()) fallback />
    }
}*/
