mod app_entry;
mod pages;
mod components;

use crate::structs::ClientSideData;
use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, Meta, MetaTags, Title};
pub use app_entry::{App, TopLevelContext};

pub fn shell(leptos_options: LeptosOptions) -> impl IntoView {
    let mut public_url : String = use_context::<ClientSideData>()
        .expect("ClientSideData should be provided, this should never fail.")
        .public_url
        .into();
    if let Some('/') = public_url.chars().last() {
        public_url.pop();
    }

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
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

