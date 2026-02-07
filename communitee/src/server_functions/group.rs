use cfg_if::cfg_if;
use leptos::prelude::*;

use crate::{
    Uuid,
    structs::{GroupData, Member},
};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ServerSideData;
    }
}

#[server]
pub async fn get_group(group_id: Uuid) -> Result<Option<GroupData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(&group_id).map(|group| group.data.clone()))
}

#[server]
pub async fn get_group_and_member(
    group_id: Uuid,
    user_id: Uuid,
) -> Result<Option<(GroupData, Member)>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(&group_id).and_then(|group| {
        group
            .data
            .members
            .get(&user_id)
            .map(|member| (group.data.clone(), member.clone()))
    }))
}

#[server]
pub async fn get_group_member(
    group_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Member>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(&group_id).and_then(|group| {
        group
            .data
            .members
            .get(&user_id)
            .map(|member| member.clone())
    }))
}
