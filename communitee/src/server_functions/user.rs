use cfg_if::cfg_if;
use chrono::{SubsecRound, Utc};
use leptos::prelude::*;

use crate::{
    Timestamp, Uuid,
    server_functions::format_datetime,
    structs::{FriendOf, GroupData, GroupInData, LoginAuth, Member, Session, UserData},
};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

#[server]
pub async fn get_user(user_id: Uuid) -> Result<Option<UserData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_user(&user_id).map(|user| user.data.clone()))
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
        .get_user(&user_id)
        .map(|user| {
            user.data
                .friends
                .iter()
                .take(max_friends)
                .flat_map(|friendship| {
                    server
                        .get_user(&friendship.user_id)
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
        .get_user(&user_id)
        .map(|user| {
            user.data
                .groups
                .iter()
                .take(max_groups)
                .flat_map(|group_id| server.get_group(group_id).map(|group| group.data.clone()))
                .collect()
        })
        .unwrap_or_default())
}
