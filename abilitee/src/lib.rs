pub mod app;
mod errors;
mod server_functions;
mod structs;

pub use app::{SubmitPost, TopLevelContext};
pub use errors::FaciliteeError;
pub use server_functions::{PerformLogin, PerformLogout, get_session_from_identity, format_datetime};
pub use structs::{ClientSideData, DefaultData, PublicUrl, ContextExt, Expect};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub use structs::{ServerSideData, InitialUserData, Server};
        pub use libertee::{TracerEngine, TracerOptions};
    }
}
