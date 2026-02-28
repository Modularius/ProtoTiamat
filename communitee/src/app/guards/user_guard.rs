use leptos::prelude::*;

use crate::{
    app::{TopLevelContext, generic_components::error_box},
    structs::ContextExt
};
use libertee::{SessionUuid, UserUuid};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[server]
#[tracing::instrument]
pub async fn get_user_id_from_session_id(
    session_id: SessionUuid
) -> Result<Option<UserUuid>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect_context();
    let server = server_side_data.server.lock()?;
    
    let session = server.get_session(&session_id)
        .ok_or_else(||ServerFnErrorErr::ServerError(format!("No Session found with id {}", session_id.to_string())))?;

    Ok(server.get_user(&session.user).map(|user| user.data.id.clone()))
}

#[component]
pub fn UserGuard<C>(
    children: TypedChildrenFn<C>,
) -> impl IntoView where C : IntoView + 'static
{
    let top_level_context = use_context::<TopLevelContext>()
        .expect_context();

    let session_id = top_level_context
        .session_id_expect();
    
    Suspend::new(async move {
        let user_id = get_user_id_from_session_id(session_id).await;
        view!{
            <ErrorBoundary fallback = error_box>
                {
                    user_id.map(|user_id| {
                        top_level_context.user_id.set(user_id);
                        children.into_inner()()
                    })
                }
            </ErrorBoundary>
        }
    })
}