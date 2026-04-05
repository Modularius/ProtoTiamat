use crate::app::{
    components::PostData,
    generic_components::{
        ButtonControl, ButtonFunction, ControlStack, LabelledInput, LabelledTextArea, SubmitControl,
    },
};
use leptos::prelude::*;
use libertee::{GroupUuid, PostUuid, UserUuid};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
use tracing::instrument;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::{ServerSideData, structs::ContextExt};
        use libertee::LiberteeError;
    }
}

#[server]
#[instrument]
pub async fn submit_post(data: SubmitPostData) -> Result<Option<PostData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let mut server = server_side_data.server.lock()?;

    let post_data = {
        match data.submmit_post_type {
            SubmitPostType::UserSelf => None,
            SubmitPostType::Reply(_post_uuid) => None,
            SubmitPostType::Group(group_uuid) => {
                let post_id = server
                    .add_post_to_group(&group_uuid, &data.user_id, data.subject, data.contents)
                    .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;
                let group = server
                    .get_group(&group_uuid)
                    .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;
                let user = server
                    .get_user(&data.user_id)
                    .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;
                let post = group
                    .get_post(&post_id)
                    .map_err(ServerFnError::<LiberteeError>::WrappedServerError)?;
                Some(PostData::new(post, user))
            }
        }
    };
    Ok(post_data)
}

#[derive(
    Default,
    Clone,
    Debug,
    EnumString,
    Display,
    EnumIter,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum SubmitPostType {
    #[default]
    #[strum(to_string = "user-self")]
    UserSelf,
    #[strum(to_string = "reply-to-post")]
    Reply(PostUuid),
    #[strum(to_string = "group")]
    Group(GroupUuid),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitPostData {
    submmit_post_type: SubmitPostType,
    user_id: UserUuid,
    subject: String,
    contents: String,
}

#[component]
#[instrument(skip_all)]
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
