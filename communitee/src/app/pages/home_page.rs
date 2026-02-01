use crate::{app::{components::{error_box, AccessBar, AdColumns, Feed, MainColumn}, TopLevelContext}, server::get_user_feed, structs::{PostData, UserData}};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
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
                            let posts = Resource::new_blocking(||(), move|_| get_user_feed(session.user.clone(), 5));
                            view!{
                                <HomePageWithUser user_data = session.user_data posts />
                            }
                        })
                    )}
                </ErrorBoundary>
            })}
        </Suspense>
    }
}

#[component]
pub fn HomePageWithUser(user_data: UserData, posts: Resource<Result<Vec<PostData>, ServerFnError>>) -> impl IntoView {
    view!{
        <MainColumn>
            <h1> "Hi there " {user_data.name.clone()} "!" </h1>
            <AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "Current feed: "</h2>
                <Suspense fallback=move || view!{}>
                    {move ||posts.get().map(|posts| view!{
                        <ErrorBoundary fallback = error_box>
                            { posts.map(|posts| view!{<Feed feed = posts.into_iter() max = 10/>}) }
                        </ErrorBoundary>
                    })}
                </Suspense>
            </AdColumns>
        </MainColumn>
    }
}
