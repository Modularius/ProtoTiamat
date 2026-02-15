use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

use crate::{RandomGeneration, Real, Timestamp, Uuid, structs::libertee::UserUuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupUuid(pub Uuid);

impl Into<GroupUuid> for String {
    fn into(self) -> GroupUuid {
        GroupUuid(self)
    }
}

impl ToString for GroupUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupData {
    pub id: GroupUuid,
    pub name: String,
    pub members: HashMap<MemberUuid, Member>,
    pub adjacent_groups: Vec<(GroupUuid, Real)>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberUuid(Uuid);

impl Into<MemberUuid> for String {
    fn into(self) -> MemberUuid {
        MemberUuid(self)
    }
}

impl ToString for MemberUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: MemberUuid,
    pub user: UserUuid,
    pub joined: Timestamp,
    pub delegates: HashMap<MemberUuid, Real>,
}

impl Member {
    pub fn new(id: MemberUuid, user: UserUuid) -> Self {
        Self {
            id,
            user,
            joined: Utc::now(),
            delegates: Default::default(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::structs::libertee::{Feed, Store, Post, PostUuid};
        use rand::seq::IndexedRandom;
        use std::ops::Bound::{Excluded, Unbounded};

        #[derive(Clone, Debug)]
        pub struct Group {
            pub(crate) data: GroupData,
            pub(crate) store: Store,
        }

        impl Group {
            pub(crate) fn new(data: GroupData) -> Self {
                Self {
                    data,
                    store: Default::default(),
                }
            }

            fn evaluate_post_for(&self, target_member: &MemberUuid, post: &Post) -> Option<Post> {
                if let Some(_member) = self.data.members.get(target_member) {

                }
                Some(post.clone())
            }

            pub(crate) fn create_feed(&self, target_member: &MemberUuid, last_post: Option<PostUuid>, max_size: usize) -> Feed {
                let posts = self.store
                    .posts
                    .range((last_post.map(Excluded).unwrap_or(Unbounded), Unbounded))
                    .filter_map(|(_,post)|self.evaluate_post_for(&target_member, &post))
                    .take(max_size)
                    .collect::<Vec<_>>();
                Feed { posts }
            }
        }

        impl GroupData {
        }

        impl RandomGeneration for GroupData {
            type Parameter = GroupUuid;
            fn new_random(id: GroupUuid) -> Self {
                Self {
                    id,
                    name: ["UK", "Music", "UK Music", "Science", "Space", "Wanking About", "Product of Inbreeding (Self Help)"]
                        .as_slice()
                        .choose(&mut rand::rng())
                        .expect("")
                        .to_string(),
                    ..Default::default()
                }
            }
        }
    }
}
