use crate::{
    app::components::{AdColumns, MainColumn, ResourceView, SessionView}, server::get_user_friends, structs::{Session, UserData}
};
use leptos::prelude::*;

#[component]
pub fn FriendlistPage() -> impl IntoView {
    || view! {
        <SessionView action = |session: &Session| {
            let session = session.clone();
            view!{ <FriendlistPageWithUser user_data = session.user_data /> }
        } />
    }
}

#[component]
pub fn FriendlistPageWithUser(
    user_data: UserData,
) -> impl IntoView {
    let friends = {
        let user_data = user_data.clone();
        Resource::new_blocking(
            move || user_data.clone(),
            |user_data| get_user_friends(user_data.id, 5)
        )
    };
    move || {
        let user_data = user_data.clone();
        view! {
            <MainColumn>
                <h1> "Hi there " {user_data.name.clone()} "!" </h1>
                //<AccessBar user_data = user_data.clone()/>
                <AdColumns>
                    <h2> "People you are friends with: "</h2>
                    <ResourceView
                        resource = friends
                        action = |friends|view! {
                            <div> "You have " {friends.len()} " friend(s)" </div>
                            <For
                                each = move ||friends.clone().into_iter().enumerate()
                                key = |(i,_)|*i
                                children = move |(_,friend)| view!{
                                    <div>
                                        <a href = format!("/user/{}", friend.id)> {friend.name} </a>
                                    </div>
                                }
                            />
                    } />
                </AdColumns>
            </MainColumn>
        }
    }
}
