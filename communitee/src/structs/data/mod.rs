use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserFeedData {
    pub datetime_feed_generated: String,
    pub posts: Vec<PostData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub author: String,
    pub author_link: String,
    pub datetime_posted: String,
    pub title: String,
    pub contents: String,
    pub replies: Vec<Self>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPageData {
    pub name: String,
    pub datetime_joined: String,
    pub properties: HashMap<String,String>,
    pub groups_in: Vec<GroupInData>,
    pub friends: Vec<FriendOf>
}

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
    friends: Vec<String>
}