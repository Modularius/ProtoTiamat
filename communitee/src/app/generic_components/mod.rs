mod button;
mod control_stack;
mod error_box;
mod inert_containers;
mod labelled_input;

pub use button::{ButtonControl, ButtonFunction, CloseButton, SubmitControl};
pub use control_stack::{Control, ControlStack, LabelledControlStack};
pub use error_box::error_box;
pub use inert_containers::{ErrorBox, RoundedBox, SharpBox};
pub use labelled_input::{LabelledInput, LabelledSelect, LabelledTextArea};
use leptos::prelude::*;

use crate::app::TopLevelContext;
use libertee::Session;

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

#[component]
pub fn SessionView<F, V>(action: F, #[prop(optional, into)] fallback: ViewFnOnce) -> impl IntoView
where
    F: Fn(&Session) -> V + Send + Sync + Clone + 'static,
    V: IntoView + 'static,
{
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;
    view! {
        <ResourceView resource = session action = move |session|session.as_ref().map(action.clone()) fallback />
    }
}
