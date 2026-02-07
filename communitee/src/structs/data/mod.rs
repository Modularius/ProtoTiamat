use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupInData {
    pub name: String,
    pub link_to_group: String,
    pub datetime_joined: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendOf {
    pub name: String,
    pub link_to_user: String,
    pub datetime_of_friendship: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friendslist {
    friends: Vec<String>,
}
