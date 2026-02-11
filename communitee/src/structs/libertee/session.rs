use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, Uuid, structs::libertee::UserData};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub uuid: Uuid,
    pub user: Uuid,
    pub user_data: UserData,
    started: Timestamp,
    ttl: TimeDelta,
}

impl Session {
    pub fn new(uuid: Uuid, user: Uuid, user_data: UserData) -> Self {
        Self {
            uuid,
            user,
            user_data,
            started: Utc::now(),
            ttl: TimeDelta::days(7),
        }
    }
}
