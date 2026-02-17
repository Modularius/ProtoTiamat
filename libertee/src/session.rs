use std::collections::HashMap;

use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{Timestamp, UserData, UserUuid, Uuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionUuid(pub Uuid);

impl Into<SessionUuid> for String {
    fn into(self) -> SessionUuid {
        SessionUuid(self)
    }
}

impl ToString for SessionUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub uuid: SessionUuid,
    pub user: UserUuid,
    pub user_data: UserData,
    started: Timestamp,
    ttl: TimeDelta,
    state: HashMap<String,String>,
}

impl Session {
    pub fn new(uuid: SessionUuid, user: UserUuid, state: HashMap<String,String>, user_data: UserData) -> Self {
        Self {
            uuid,
            user,
            user_data,
            started: Utc::now(),
            ttl: TimeDelta::days(7),
            state
        }
    }

    pub fn get_state(&self) -> HashMap<String,String> {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: HashMap<String,String>) {
        self.state = state;
    }
}
