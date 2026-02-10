use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Uuid, app::generic_components::{Control, ControlStack, LabelledInput, LabelledTextArea}};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, structs::Post};
} }

#[server]
pub async fn submit_post(data: SubmitPostData) -> Result<(), ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let mut server = server_side_data.server.lock()?;
    
    let user_id = server
        .get_user(&data.user_id)
        .map(|user|user.data.id.clone());
    if let Some(group_id) = data.group_id {
        if let Some(group) = server.get_group_mut(&group_id) {
            if let Some(user_id) = user_id {
                group.feed.add_post(user_id, data.subject, data.contents);
            }
        }
    } else {
        if let Some(user) = server.get_user_mut(&data.user_id) {
            user.feed.add_post(data.user_id, data.subject, data.contents);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitPostData {
    user_id: String,
    group_id: Option<String>,
    subject: String,
    contents: String
}

#[component]
pub fn NewPostBox(user_id: Uuid, group_id: Option<Uuid>) -> impl IntoView {
    let submit_post = ServerAction::<SubmitPost>::new();
    view! {
        <ActionForm action = submit_post>
            <input name = "data[user_id]" hidden = true value = {user_id} />
            <input name = "data[group_id]" hidden = true value = {group_id} />
            <div class = "post new-post">
                <div class = "post-inner new-post-inner">
                    <ControlStack>
                        <LabelledInput name = "data[subject]" label = "Subject: " typ = "text" value = "" />
                    </ControlStack>
                    <ControlStack>
                        <LabelledTextArea name = "data[contents]" label = "Content: "  value = "" />
                    </ControlStack>
                    <ControlStack>
                        <Control>
                            <input type = "submit" value = "Submit" />
                        </Control>
                        <Control>
                            <input type = "button" value = "Clear" />
                        </Control>
                    </ControlStack>
                </div>
            </div>
        </ActionForm>
    }
}
