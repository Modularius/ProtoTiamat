use crate::{
    app::{
        components::{AdColumns, MainColumn, ResourceView, SessionView},
    }, server::get_user_groups, structs::{Session, UserData}
};
use leptos::prelude::*;

#[component]
pub fn GroupslistPage() -> impl IntoView {
    || view! {
        <SessionView action = |session: &Session| {
            let session = session.clone();
            view!{ <GroupslistPageWithUser user_data = session.user_data /> }
        } />
    }
}

#[component]
pub fn GroupslistPageWithUser(user_data: UserData) -> impl IntoView {
    let groups = {
        let user_data = user_data.clone();
        Resource::new_blocking(
            move || user_data.clone(),
            |user_data| get_user_groups(user_data.id, 5)
        )
    };
    move || {
        let user_data = user_data.clone();
        view! {
            <MainColumn>
                <h1> "Hi there " {user_data.name.clone()} "!" </h1>
                //<AccessBar user_data = user_data.clone()/>
                <AdColumns>
                    <h2> "Groups you are currently subscribed to or following: "</h2>
                    <ResourceView
                        resource = groups
                        action = |groups|view!{
                            <div> "You have membership of " {groups.len()} " group(s)" </div>
                            <For
                                each = move ||groups.clone().into_iter().enumerate()
                                key = |(i,_)|*i
                                children = move |(_,group)| view!{
                                    <div>
                                        <a href = format!("/group/{}", group.id)> {group.name} </a>
                                    </div>
                                }
                            />
                        }
                    />
                </AdColumns>
            </MainColumn>
        }
    }
}
