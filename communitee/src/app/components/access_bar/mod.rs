mod user_settings;

use leptos::{either::Either, ev::MouseEvent, prelude::*};
use leptos_router::hooks::use_navigate;
pub use user_settings::UserSettings;

use crate::{app::{TopLevelContext, components::{ResourceView, SessionView}}, server::get_user_friends, structs::UserData};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum ShowAccessBarSubBar {
    #[default]
    None,
    Friends,
    Groups,
    Settings,
    Favourites,
}

#[derive(Default, Clone, Debug)]
pub struct AccessBarContext {
    sub_bar: RwSignal<ShowAccessBarSubBar>,
}

#[component]
pub fn AccessBar(user_data: UserData) -> impl IntoView {
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;

    let context = AccessBarContext::default();
    let sub_bar = context.sub_bar;
    provide_context(context);

    let on_click = move |current_sub_bar: ShowAccessBarSubBar| {
        if current_sub_bar == sub_bar.get() {
            sub_bar.set(ShowAccessBarSubBar::None)
        } else {
            sub_bar.set(current_sub_bar)
        }
    };

    view! {
        <div class = "access-bar">
            <div class = "access-bar-inner">
                <div class = "button friends" on:click = move |_|on_click(ShowAccessBarSubBar::Friends)> Friends </div>
                <div class = "button groups" on:click = move |_|on_click(ShowAccessBarSubBar::Groups)> Groups </div>
                <div class = "button posts" on:click = move|_|use_navigate()("/posts", Default::default())> Posts </div>
                <div class = "button faves" on:click = move |_|on_click(ShowAccessBarSubBar::Favourites)> Favourites </div>
                <div class = "button settings" on:click = move |_|on_click(ShowAccessBarSubBar::Settings)> User Settings </div>
                <div class = "button user-logout" on:click = move|_|{/*logout()*/}> Logout </div>
            </div>
        </div>

        <Show when = move ||matches!(sub_bar.get(), ShowAccessBarSubBar::Friends)>
            <div class = "friends access-sub-bar">
                <div class = "friends access-sub-bar-inner">
                    <Friends/>
                </div>
            </div>
        </Show>

        <Show when = move ||matches!(sub_bar.get(), ShowAccessBarSubBar::Groups)>
            <div class = "groups access-sub-bar">
                <div class = "groups access-sub-bar-inner">
                </div>
            </div>
        </Show>

        <Show when = move ||matches!(sub_bar.get(), ShowAccessBarSubBar::Settings)>
            <UserSettings user_data = user_data.clone()/>
        </Show>

        <Show when = move ||matches!(sub_bar.get(), ShowAccessBarSubBar::Favourites)>
            <div class = "favourites access-sub-bar">
                <div class = "favourites access-sub-bar-inner">
                </div>
            </div>
        </Show>
    }
}

#[component]
fn Friends() -> impl IntoView {
    view!{
        <SessionView action = |session| {
            let session = session.clone();
            let friends = Resource::new_blocking(||(), move |_| get_user_friends(session.user.clone(), 5));
            view!{
                <ResourceView resource = friends action = |friends| view!{
                    <For
                        each = move ||friends.clone().into_iter().enumerate()
                        key = |(i,_)|*i
                        children = move |(_,friend)| view!{ <Friend friend/> }
                    />
                }/>
            }
        } />
    }
}

#[component]
fn Friend(friend: UserData) -> impl IntoView {
    view!{
        <div class = "friend">
            {friend.name}
        </div>
    }
}
