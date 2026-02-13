#![allow(unused_crate_dependencies)]
#![recursion_limit = "256"]

mod app;
mod server_functions;
mod structs;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};

pub use app::{App, shell, SubmitPost};
pub use structs::{ClientSideData, DefaultData, PublicUrl, RandomGeneration};

/// Used by instances of the website to refer to server-side sessions.
pub type Uuid = String;
/// The timestamp type with timezone.
pub type Timestamp = DateTime<Utc>;
pub type Real = f64;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use structs::{ServerSideData, Server};
        use rand::seq::IteratorRandom;
        
        pub trait Uuidlike {
            fn generate_random(size: usize) -> Self;
        }

        impl Uuidlike for Uuid {
            fn generate_random(size: usize) -> Self {
                let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
                (0..size).map(|_|
                    alphabet.iter()
                        .choose(&mut rand::rng())
                        .to_owned()
                        .unwrap()
                    ).collect::<Uuid>()
            }
        }
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
