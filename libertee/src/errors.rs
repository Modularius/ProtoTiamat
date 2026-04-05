use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{GroupUuid, LoginAuth, PostUuid, SessionUuid, UserUuid};

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum LiberteeError {
    #[error("No User found with id {}", 0.to_string())]
    NoUserFound(UserUuid),
    #[error("No Group found with id {}", 0.to_string())]
    NoGroupFound(GroupUuid),
    #[error("No Session found with id {}", 0.to_string())]
    NoSessionFound(SessionUuid),
    #[error("No Post found with id {}", 0.to_string())]
    NoPostFound(PostUuid),
    #[error("No credentials found with authorisation {0:?}")]
    NoCredentialsFound(LoginAuth),
    #[error("Credentials found with authorisation{auth:?}, but no user found with id {}.", user_id.to_string())]
    CredentialsButNoUserFound { auth: LoginAuth, user_id: UserUuid },
}
