use crate::{
    app::components::{AccessBar, AdColumns, MainColumn, ResourceView, SessionView},
    server::get_user_friends,
    structs::{Session, UserData},
};
use leptos::prelude::*;

#[component]
pub fn FriendlistPage() -> impl IntoView {
    view! {
        <SessionView fallback = move || view! {}
            action = |session: &Session| {
                let session = session.clone();
                let friends =
                    Resource::new_blocking(|| (), move |_| get_user_friends(session.user.clone(), 5));
                view! {
                    <FriendlistPageWithUser user_data = session.user_data.clone() friends/>
                }
            }
        />
    }
}

#[component]
pub fn FriendlistPageWithUser(
    user_data: UserData,
    friends: Resource<Result<Vec<UserData>, ServerFnError>>,
) -> impl IntoView {
    let action = |friends: Vec<UserData>| {
        view! {
            <div> "You have " {friends.len()} " friend(s)" </div>
            <For
                each = move ||friends.clone().into_iter().enumerate()
                key = |(i,_)|*i
                children = move |(_,friend)| view!{ <div> <a href = format!("user/{}", friend.id)> {friend.name} </a> </div> }
            />
        }
    };
    view! {
        <MainColumn>
            <h1> "Hi there " {user_data.name.clone()} "!" </h1>
            <AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "People you are friends with: "</h2>
                <ResourceView resource = friends action />
            </AdColumns>
        </MainColumn>
    }
}
