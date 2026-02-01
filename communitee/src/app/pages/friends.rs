use crate::{app::{components::{error_box, AccessBar, AdColumns, MainColumn}, TopLevelContext}, server::get_user_friends, structs::UserData};
use leptos::prelude::*;


#[component]
pub fn FriendlistPage() -> impl IntoView {
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;
    
    let no_user = move || view! {};
    view!{
        <Suspense fallback=no_user> {move ||
            session.get().map(|session| view!{
                <ErrorBoundary fallback = error_box>
                    {session.map(|session|
                        session.map(|session| {
                            let friends = Resource::new_blocking(||(), move|_| get_user_friends(session.user.clone(), 5));
                            view!{
                                <FriendlistPageWithUser user_data = session.user_data friends/>
                            }
                        })
                    )}
                </ErrorBoundary>
            })}
        </Suspense>
    }
}

#[component]
pub fn FriendlistPageWithUser(user_data: UserData, friends: Resource<Result<Vec<UserData>, ServerFnError>>) -> impl IntoView {
    view!{
        <MainColumn>
            <h1> "Hi there " {user_data.name.clone()} "!" </h1>
            <AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "People you are friends with: "</h2>
                <Suspense fallback=move || view!{}>
                    {move ||friends.get().map(|friends| view!{
                        <ErrorBoundary fallback = error_box>
                            { friends.map(|friends| view!{
                                <div> "You have " {friends.len()} " friend(s)" </div>
                                <For
                                    each = move ||friends.clone().into_iter().enumerate()
                                    key = |(i,_)|*i
                                    children = move |(_,friend)| view!{ <div> {friend.name} </div> }
                                />
                            }) }
                        </ErrorBoundary>
                    })}
                </Suspense>
            </AdColumns>
        </MainColumn>
    }
}
