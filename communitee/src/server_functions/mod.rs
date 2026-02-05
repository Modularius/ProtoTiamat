use cfg_if::cfg_if;
use chrono::{SubsecRound, Utc};
use leptos::prelude::*;

use crate::{
    Timestamp, Uuid, structs::{FriendOf, GroupData, GroupInData, LoginAuth, Member, PostData, Session, UserData, UserFeedData, UserPageData}
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
) -> Result<Option<UserFeedData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server
        .get_user(&user_id)
        .map(|user|
            UserFeedData {
                datetime_feed_generated: format_datetime(&Utc::now()),
                posts: user.feed
                    .posts
                    .iter()
                    .take(max_posts)
                    .flat_map(|post| {
                        server.get_user(&post.data.author)
                            .map(|author_user|
                                PostData {
                                    author: author_user.data.name.clone(),
                                    author_link: format!("/user/{}", author_user.data.id),
                                    datetime_posted: format_datetime(&post.data.posted_at),
                                    title: post.data.title.clone(),
                                    contents: post.data.content.clone(),
                                    replies: Default::default(),
                                }
                            )
                    })
                    .collect()
            }
        )
    )
}

#[server]
pub async fn get_group(
    group_id: Uuid
) -> Result<Option<GroupData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(&group_id).map(|group|group.data.clone()))
}

#[server]
pub async fn get_group_and_member(
    group_id: Uuid,
    user_id: Uuid
) -> Result<Option<(GroupData,Member)>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;
    Ok(server.get_group(&group_id)
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
    Ok(server.get_group(&group_id)
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
    Ok(server.get_user(&user_id).map(|user|user.data.clone()))
}

fn format_datetime(datetime: &Timestamp) -> String {
    let date = datetime.date_naive();
    let time = datetime.time().trunc_subsecs(0);
    format!("{}, {}", date.to_string(), time.to_string())
}

#[server]
pub async fn get_user_page_data(
    user_id: Option<Uuid>
) -> Result<Option<UserPageData>, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;

    let user_page_data = user_id
        .and_then(|user_id|server.get_user(&user_id))
        .map(|user| {
        let properties = user.data
            .properties
            .clone();
        let groups_in = user.data
            .groups
            .iter()
            .flat_map(|group_id| server
                    .get_group(group_id)
                    .and_then(|group| group.data
                        .members
                        .get(&user.data.id)
                        .map(|member| GroupInData {
                            name: group.data.name.clone(),
                            link_to_group: format!("/group/{}", group.data.id),
                            datetime_joined: format_datetime(&member.joined)
                        })
                    )
            ).collect();
        let friends = user.data
            .friends
            .iter()
            .flat_map(|friendship| server
                .get_user(&friendship.user_id)
                .map(|friend| FriendOf {
                        name: friend.data.name.clone(),
                        link_to_user: format!("/user/{}",friend.data.id),
                        datetime_of_friendship: format_datetime(&friendship.datetime_of_friendship),
                    })
            ).collect();

        UserPageData {
            name: user.data.name.clone(),
            datetime_joined: format_datetime(&user.data.datetime_joined),
            properties,
            groups_in,
            friends
        }
    });
    Ok(user_page_data)
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
                .flat_map(|group_id| {
                    server
                        .get_group(group_id)
                        .map(|group| group.data.clone())
                })
                .collect()
        })
        .unwrap_or_default())
}


