use cfg_if::cfg_if;
use leptos::prelude::*;

use crate::{
    Uuid,
    structs::{GroupData, LoginAuth, Member, PostData, Session, UserData},
};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

pub async fn require_login() -> Result<Option<Session>, ServerFnError> {
    if let Some(session) = perform_login(LoginAuth::default(), "".into()).await? {
        Ok(Some(session))
    } else {
        #[cfg(feature = "hydrate")]
        {
            use leptos_router::hooks::use_navigate;
            let nav = use_navigate();
            nav(&format!("/login"), Default::default());
        }

        Ok(None)
    }
}
/*
#[server]
pub async fn get_session() -> Result<Option<Session>,ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");

    let server = server_side_data.server.lock()?;
    if let Some(server) = server.get_session(auth) .as_ref() {
        Ok(server.get_session())
    } else {
        Ok(None)
    }
}
 */

#[server]
pub async fn perform_login(
    auth: LoginAuth,
    new_path: String,
) -> Result<Option<Session>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");

    let server = server_side_data.server.lock()?;
    Ok(server.get_session(&auth).cloned())
    //let nav = use_navigate();
    //nav(&new_path, Default::default());
    //Ok(())
}

#[server]
pub async fn get_user_feed(
    user_id: Uuid,
    max_posts: usize,
) -> Result<Vec<PostData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server
        .get_user(user_id)
        .map(|user| {
            user.feed
                .posts
                .iter()
                .take(max_posts)
                .map(|post| post.data.clone())
                .collect()
        })
        .unwrap_or_default())
}

#[server]
pub async fn get_group(
    group_id: Uuid
) -> Result<Option<GroupData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(group_id).map(|group|group.data.clone()))
}

#[server]
pub async fn get_group_and_member(
    group_id: Uuid,
    user_id: Uuid
) -> Result<Option<(GroupData,Member)>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(group_id)
        .and_then(|group|
            group.data
                .members
                .get(&user_id)
                .map(|member|
                    (group.data.clone(), member.clone())
            )
        )
    )
}

#[server]
pub async fn get_group_member(
    group_id: Uuid,
    user_id: Uuid
) -> Result<Option<Member>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(group_id)
        .and_then(|group|
            group.data
                .members
                .get(&user_id)
                .map(|member|
                    member.clone()
            )
        )
    )
}

#[server]
pub async fn get_user(
    user_id: Uuid
) -> Result<Option<UserData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_user(user_id).map(|user|user.data.clone()))
}

#[server]
pub async fn get_user_friends(
    user_id: Uuid,
    max_friends: usize,
) -> Result<Vec<UserData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server
        .get_user(user_id)
        .map(|user| {
            user.data
                .friends
                .iter()
                .take(max_friends)
                .flat_map(|friend_id| {
                    server
                        .get_user(friend_id.clone())
                        .map(|friend| friend.data.clone())
                })
                .collect()
        })
        .unwrap_or_default())
}

#[server]
pub async fn get_user_groups(
    user_id: Uuid,
    max_groups: usize,
) -> Result<Vec<GroupData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server
        .get_user(user_id)
        .map(|user| {
            user.data
                .groups
                .iter()
                .take(max_groups)
                .flat_map(|group_id| {
                    server
                        .get_group(group_id.clone())
                        .map(|group| group.data.clone())
                })
                .collect()
        })
        .unwrap_or_default())
}


