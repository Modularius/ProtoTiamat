mod app_entry;
mod guards;
mod components;
mod generic_components;
mod pages;

use crate::structs::{ClientSideData, ContextExt};
pub use app_entry::{App, TopLevelContext};
pub use components::SubmitPost;
use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, Meta, MetaTags, Title, provide_meta_context};

pub fn shell(leptos_options: LeptosOptions) -> impl IntoView {
    let public_url: String = use_context::<ClientSideData>()
        .expect_context()
        .public_url
        .header_form();
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options.clone() root = public_url.clone() />
                <HashedStylesheet options=leptos_options root = public_url />
                <MetaTags/>
                // sets the document title
                <Title text="Communitee" />

                // injects metadata in the <head> of the page
                <Meta charset="UTF-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
