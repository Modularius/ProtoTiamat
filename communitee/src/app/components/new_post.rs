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
        let post = Option::zip(server.get_group_mut(&group_id),user_id)
            .and_then(|(group, user_id)| {
                let id = group.store.add_post(user_id.clone(), data.subject, data.contents);
                group.store.get_post_mut(id)
            });
        let post_author = post.map(|post|post.data.author.clone());
        let user = post_author.and_then(|post_author|server.get_user(&post_author));
        Option::zip(post,user).map(|(post, user)|PostData::new(post, user))
    } else {
        server.get_user_mut(&data.user_id)
            .and_then(|user| {
                let store = &mut user.store;
                let post_id = user.store
                    .add_post(data.user_id, data.subject, data.contents);
                if let Some(post) = user.store.get_post_mut(post_id) {
                    Some(PostData::new(&post, user))
                } else {
                    None
                }
            }
        )
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
pub fn NewPostBox(user_id: UserUuid, group_id: Option<GroupUuid>) -> impl IntoView {
    let submit_post = ServerAction::<SubmitPost>::new();
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
