use crate::{
    app::{
        TopLevelContext,
        components::{AccessBar, AdColumns, MainColumn, error_box},
    },
    structs::UserData,
};
use leptos::prelude::*;

#[component]
pub fn GroupPage() -> impl IntoView {
    let top_level_context = use_context::<TopLevelContext>()
        .expect("TopLevelContext should be provided, this should never fail.");
    let session = top_level_context.session;

    let no_user = move || view! {};
    view! {
        <Suspense fallback=no_user> {move ||
            session.get().map(|session| view!{
                <ErrorBoundary fallback = error_box>
                    {session.map(|session|session.map(|session| view!{ <GroupsPageWithUser user_data = session.user_data /> }))}
                </ErrorBoundary>
            })}
        </Suspense>
    }
}

#[component]
pub fn GroupsPageWithUser(user_data: UserData) -> impl IntoView {
    view! {
        <MainColumn>
            <h1> "Hi there " {user_data.name.clone()} "!" </h1>
            <AccessBar user_data = user_data.clone()/>
            <AdColumns>
                <h2> "Groups you are currently subscribed to or following: "</h2>
                //<Feed feed = ExampleFeed{num: 0} max = 10/>
            </AdColumns>
        </MainColumn>
    }
}
