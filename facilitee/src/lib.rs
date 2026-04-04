mod app;
mod errors;
mod server_functions;
mod structs;
use cfg_if::cfg_if;

pub use app::{App, SubmitPost, shell, TopLevelContext};
pub use errors::FaciliteeError;
pub use structs::{ClientSideData, DefaultData, PublicUrl};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use structs::{ServerSideData, InitialUserData, Server, SessionStorage};
        pub use libertee::{TracerEngine, TracerOptions};
    }
}