use leptos::{prelude::*, server_fn::ServerFn};

use crate::{
    app::{TopLevelContext, generic_components::error_box},
    structs::ContextExt
};
use libertee::SessionUuid;

#[component]
pub fn PageGuard<S, P, C>(
    with_parameters: P,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    C : IntoView + 'static,
    Result<View<C>, <S as ServerFn>::Error>: IntoView,
    P : Fn(SessionUuid)->S + Send + 'static,
    S : ServerFn + Clone + Send + Sync + 'static,
    <S as ServerFn>::Output : Clone + Sync + 'static,
    <S as ServerFn>::Error : Clone
{
    let server_action = ServerAction::<S>::new();
    move || {
        let children = children.clone();
        let session_id = use_context::<TopLevelContext>()
            .expect_context()
            .session_id_expect();
        server_action.dispatch(with_parameters(session_id));
        Suspend::new(async move {
            server_action.value()
                .get()
                .map(|value|
                    view!{
                        <ErrorBoundary fallback = error_box>
                        {
                            value.map(|value| {
                                provide_context(value);
                                children.into_inner()()
                            })
                        }
                        </ErrorBoundary>
                    }
                )
        })
    }
    
    // Suspend::new(async move {
    //     let user_id = get_user_id_from_session_id(session_id).await;
    //     view!{
    //         <ErrorBoundary fallback = error_box>
    //             {
    //                 user_id.map(|user_id| {
    //                     top_level_context.user_id.set(user_id);
    //                     children.into_inner()()
    //                 })
    //             }
    //         </ErrorBoundary>
    //     }
    // })
}