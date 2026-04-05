pub mod app;
mod errors;
mod server_functions;
mod structs;

pub use app::{SubmitPost, TopLevelContext};
pub use errors::AbiliteeError;
pub use server_functions::{
    PerformLogin, PerformLogout, format_datetime, get_session_from_identity,
};
pub use structs::{ClientSideData, ContextExt, DefaultData, Expect, PublicUrl};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use structs::{ServerSideData, InitialUserData, Server};
        pub use libertee::{TracerEngine, TracerOptions};
    }
}
