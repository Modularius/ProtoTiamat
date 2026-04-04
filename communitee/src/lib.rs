#![allow(unused_crate_dependencies)]
#![recursion_limit = "256"]

mod errors;

use cfg_if::cfg_if;

pub use errors::CommuniteeError;
pub use facilitee::{
    App, SubmitPost, shell,
    FaciliteeError,
    ClientSideData, DefaultData, PublicUrl,
};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use facilitee::{ServerSideData, InitialUserData, Server, SessionStorage};
        pub use libertee::{TracerEngine, TracerOptions};
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use facilitee::TopLevelContext;
    use leptos::prelude::use_context;

    console_error_panic_hook::set_once();
    wasm_tracing::set_as_global_default();

    leptos::mount::hydrate_body(App);

    let client_side_data = use_context::<TopLevelContext>()
        .expect("TopLevelContext should exists, this should never fail.")
        .client_side_data;

    let public_url: &'static str = client_side_data.public_url.hydrate_form();
    leptos::server_fn::client::set_server_url(public_url);
}
