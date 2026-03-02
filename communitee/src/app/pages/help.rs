use crate::app::{components::{FootBar, MainColumn, TopBar}, generic_components::RoundedBox, guards::SessionGuard};
use leptos::prelude::*;


#[component]
pub fn HelpPage() -> impl IntoView {
    view! {
        <SessionGuard>
            <TopBar/>
            <MainColumn>
                <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
                <h2 class = "text-xl m-2"> "The social media platform exclusively controlled by its users." </h2>
                <RoundedBox>
                    <h3 class = "text-lg m-2"> "Using Communitee guarantees:" </h3>
                    <ul class = "text-sm m-2">
                        <li> "Your content and data is *never* used to personalised your feed or the adverts you are shown." </li>
                        <li> "Your experience is curated by yourself and fellow users, and never by an opaque algorithm controlled by tech companies." </li>
                        <li> "You and your fellow users can anonymously vote for the content you like, and this vote exclusively determines which content is shown. There are no paid posts." </li>
                        <li> "All adverts are clearly marked as adverts, and are chosen by the users." </li>
                        <li> "Admins are democratically elected by the users they serve." </li>
                        <li> "Content is moderated by fellow users who are empowered by the democratic wishes of the users they serve." </li>
                        <li> "All users are verified in a safe and anonymous process, which guarantees identity without risking their private data." </li>
                        <li> "Data is distributed among many cooperating nodes, with multiple levels of encryption to ensure privacy." </li>
                    </ul>
                </RoundedBox>
            </MainColumn>
            <FootBar />
        </SessionGuard>
    }
}
