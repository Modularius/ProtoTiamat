use crate::{
    Uuid,
    app::components::{AdColumns, MainColumn},
    app::generic_components::{ResourceView, SessionView},
    structs::Session,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::{ServerSideData, structs::User};
} }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FriendData {
    name: String,
    link: String,
}

#[cfg(feature = "ssr")]
impl FriendData {
    fn from(friend: &User) -> Self  {
        Self {
            name: friend.data.name.clone(),
            link: format!("/user/{}", friend.data.id),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendslistPageData {
    user_name: String,
    friends: Vec<FriendData>,
}

#[server]
async fn get_friendslist_page_data(
    session: Session,
    max_friends: usize,
) -> Result<FriendslistPageData, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>()
        .expect("ServerSideData should be provided, this should never fail.");
    let server = server_side_data.server.lock()?;

    let data = FriendslistPageData {
        user_name: session.user_data.name.clone(),
        friends: server.get_user(&session.user)
            .map(|user| user.data
                .friends
                .iter()
                .take(max_friends)
                .flat_map(|friendship| {
                    server.get_user(&friendship.user_id)
                        .map(|friend| FriendData::from(friend))
                })
                .collect()
            )
            .unwrap_or_default(),
    };
    
    Ok(data)
}

#[component]
pub fn FriendlistPage() -> impl IntoView {
    || {
        view! {
            <SessionView action = |session: &Session| {
                let session = session.clone();
                let friendslist_page_data = Resource::new_blocking(
                    move ||session.clone(),
                    |session| get_friendslist_page_data(session, 10)
                );
                view!{
                    <ResourceView
                        resource = friendslist_page_data
                        action = |friendslist_page_data|
                            FriendlistPageWithData(FriendlistPageWithDataProps{ friendslist_page_data })
                    />
                }
            } />
        }
    }
}

#[component]
pub fn FriendlistPageWithData(friendslist_page_data: FriendslistPageData) -> impl IntoView {
    view! {
        <h2> {format!("You have {} friend(s)", friendslist_page_data.friends.len())} </h2>
        <For
            each = move ||friendslist_page_data.friends.clone().into_iter().enumerate()
            key = |(i,_)|*i
            children = move |(_,friend)| view!{
                <div>
                    <a href = {friend.link}> {friend.name} </a>
                </div>
            }
        />
    }
}
