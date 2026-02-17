mod public_url;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

//pub use libertee::{GroupData, GroupUuid, LoginAuth, Session, SessionUuid, UserData, UserUuid, PostUuid, RandomGeneration};
pub use public_url::PublicUrl;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use clap::{Args};
        use std::sync::{Arc, Mutex};
        use std::collections::HashMap;
        pub use libertee::{User, Server, Post, Feed, Member, Group, SessionUuid};
        use actix_session::storage::{LoadError, SessionKey, SessionStore};

        /// Encapsulates all run-time settings which are only available to the server.
        #[derive(Default, Clone)]
        pub struct ServerSideData {
            pub server: Arc<Mutex<Server>>,
        }
        
        impl SessionStore for ServerSideData {
            fn load(
                &self,
                session_key: &SessionKey,
            ) -> impl Future<Output = Result<Option<HashMap<String, String>>, LoadError>> {
                async {
                    if let Ok(server) = self.server.lock() {
                        Ok(server.get_session(&SessionUuid(session_key.as_ref().to_string())).map(|session|session.get_state()))
                    } else {
                        Ok(None)
                    }
                }
            }

            fn save(
                &self,
                session_state: HashMap<String, String>,
                ttl: &Duration,
            ) -> impl Future<Output = Result<SessionKey, SaveError>> {
                async {
                    if let Ok(server) = self.server.lock() {
                        Ok(server.cre get_session(&SessionUuid(session_key.as_ref().to_string())).map(|session|session.set_state()))
                    } else {
                        Ok(None)
                    }
                }
            }

            fn update(
                &self,
                session_key: SessionKey,
                session_state: HashMap<String, String>,
                ttl: &Duration,
            ) -> impl Future<Output = Result<SessionKey, UpdateError>> {

            }

            fn update_ttl(
                &self,
                session_key: &SessionKey,
                ttl: &Duration,
            ) -> impl Future<Output = Result<(), Error>> {

            }

            fn delete(
                &self,
                session_key: &SessionKey,
            ) -> impl Future<Output = Result<(), Error>> {
                
            }
        }


        /// Contains the settings defined in the CLI used as default values in the UI's inputs.
        #[derive(Default, Clone, Debug, Serialize, Deserialize, Args)]
        pub struct InitialUserData {
            #[clap(long)]
            pub initial_user_name: String,

            #[clap(long)]
            pub initial_user_username: String,
            
            #[clap(long)]
            pub initial_user_password: String
        }
    }
}

/// Contains the settings defined in the CLI used as default values in the UI's inputs.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Args))]
pub struct DefaultData {}


/// Encapsulates all run-time settings which are available to the client.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientSideData {
    pub default_data: DefaultData,
    pub public_url: PublicUrl,
}
