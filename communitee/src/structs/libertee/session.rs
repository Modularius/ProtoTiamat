use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{structs::libertee::UserData, Timestamp, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub user: Uuid,
    pub user_data: UserData,
    started: Timestamp,
    ttl: TimeDelta,
}

impl Session {
    pub fn new(user: Uuid, user_data: UserData) -> Self {
        Self {
            user,
            user_data,
            started: Utc::now(),
            ttl: TimeDelta::days(7),
        }
    }
}