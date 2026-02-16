use leptos::prelude::*;
use libertee::{GroupUuid, Post, PostUuid, UserUuid};
use serde::{Deserialize, Serialize};

use crate::app::{components::PostData, generic_components::{
    ButtonControl, ButtonFunction, ControlStack, LabelledInput, LabelledTextArea, SubmitControl,
}};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::ServerSideData;
} }

#[server]
pub async fn submit_post(data: SubmitPostData) -> Result<Option<PostData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let mut server = server_side_data.server.lock()?;

    let user_id = server
        .get_user(&data.user_id)
        .map(|user| user.data.id.clone());
    let post = if let Some(group_id) = data.group_id {
        if let Some(group) = server.get_group_mut(&group_id) {
            user_id.and_then(|user_id| {
                let post_id = group.store.add_post(user_id, data.subject, data.contents);
                server.get_user(&user_id)
                    .and_then(|user|
                        group.store.get_post_mut(post_id).map(|post|PostData::new(post, user))
                    )
            })
        } else {
            None
        }
    } else {
        server.get_user_mut(&data.user_id)
            .and_then(|user| {
            let post_id = user.store
                .add_post(data.user_id, data.subject, data.contents);
            user.store.get_post_mut(post_id).map(|post|PostData::new(&post, user))
        })
    };
    Ok(post)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitPostData {
    user_id: UserUuid,
    group_id: Option<GroupUuid>,
    subject: String,
    contents: String,
}

#[component]
pub fn NewPostBox(user_id: UserUuid, group_id: Option<GroupUuid>, posts: RwSignal<Vec<RwSignal<PostData>>>, datetime_feed_generated: RwSignal<String>) -> impl IntoView {
    let submit_post = ServerAction::<SubmitPost>::new();
    Effect::new(move |_| {
        if let Some(Ok(Some(post))) = submit_post.value().get() {
            post.data.author
            let posts = posts.get().insert(0, RwSignal::new());
            posts.set()
        }
    });
    view! {
        <ActionForm action = submit_post>
            <input name = "data[user_id]" hidden = true value = {user_id.to_string()} />
            <input name = "data[group_id]" hidden = true value = {group_id.map(|group_id|group_id.to_string())} />
            <div class = "post new-post">
                <div class = "post-inner new-post-inner">
                    <ControlStack>
                        <LabelledInput name = "data[subject]" label = "Subject: " typ = "text" value = "" />
                    </ControlStack>
                    <ControlStack>
                        <LabelledTextArea name = "data[contents]" label = "Content: "  value = "" />
                    </ControlStack>
                    <ControlStack>
                        <SubmitControl value = "Submit" />
                        <ButtonControl value = "Clear" on_click = ButtonFunction::Closure(Box::new(|_ev|{})) />
                    </ControlStack>
                </div>
            </div>
        </ActionForm>
    }
}
