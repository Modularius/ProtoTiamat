#![allow(unused_crate_dependencies)]
#![recursion_limit = "256"]

mod app;
mod server_functions;
mod structs;

use cfg_if::cfg_if;

pub use app::{App, SubmitPost, shell};
pub use structs::{ClientSideData, DefaultData, PublicUrl};
//use libertee::{Real, Timestamp, RandomGeneration};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use structs::{ServerSideData, Server};
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::TopLevelContext;
    use leptos::prelude::use_context;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);

    let client_side_data = use_context::<TopLevelContext>()
        .expect("TopLevelContext should exists, this should never fail.")
        .client_side_data;

    // The `leak` consumes the `String`, marks it's heap allocation as `'static`
    // and returns a static reference to it.
    // This only results in an actual memory leak if the returned reference is ever dropped.
    // By passing it to `set_server_url` we ensure this doesn't happen until the app is closed.
    // Maybe, one day, leptos will allow `set_server_url` to be a String, allowing us to avoid
    // having to use this scary sounding `leak` method... but this is not that day.
    let public_url: &'static str = client_side_data.public_url.hydrate_form();
    leptos::server_fn::client::set_server_url(public_url);
}
