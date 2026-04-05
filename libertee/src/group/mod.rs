mod member;
//mod policy;
//mod history;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use crate::{LiberteeError, RandomGeneration, Real, UserUuid, Uuid};

pub use member::{Delegate, Member, MemberUuid};
//pub use history::GroupHistory;
//pub use policy::GroupAdmin;

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupUuid(pub Uuid);

impl Into<GroupUuid> for String {
    fn into(self) -> GroupUuid {
        GroupUuid(self)
    }
}


impl Display for GroupUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupData {
    pub id: GroupUuid,
    pub name: String,
    pub members: HashMap<MemberUuid, Member>,
    //pub admins: HashMap<MemberUuid, GroupAdmin>,
    pub member_by_user_id: HashMap<UserUuid, MemberUuid>,
    pub adjacent_groups: Vec<(GroupUuid, Real)>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::{Feed, Store, Post, PostUuid};
        use rand::seq::IndexedRandom;
        use std::ops::Bound::{Excluded, Unbounded};

        #[derive(Clone, Debug)]
        pub struct Group {
            pub data: GroupData,
            pub store: Store,
            //pub history: Vec<GroupHistory>,
        }

        impl Group {
            pub fn new(data: GroupData) -> Self {
                Self {
                    data,
                    store: Default::default(),
                    //history: Default::default(),
                }
            }

            pub fn get_member_id_from_user_id(&self, user_id: &UserUuid) -> Option<&MemberUuid> {
                self.data.member_by_user_id.get(user_id)
            }


            pub(crate) fn add_member(&mut self, user_id: UserUuid) {
                let member_id = MemberUuid(format!("{}", self.data.members.len()));
                self.data
                    .members
                    .insert(member_id.clone(), Member::new(member_id.clone(), user_id.clone()));
                self.data.member_by_user_id.insert(user_id, member_id);
            }

            fn evaluate_post_for(&self, target_member: &MemberUuid, post: &Post) -> Option<Post> {
                if let Some(_member) = self.data.members.get(target_member) {

                }
                Some(post.clone())
            }

            pub fn get_post(&self, post_id: &PostUuid) -> Result<&Post, LiberteeError> {
                self.store.get_post(post_id)
                    .ok_or_else(|| LiberteeError::NoPostFound(post_id.clone()))
            }

            pub fn create_feed(&self, target_member: &MemberUuid, last_post: Option<PostUuid>, max_size: usize) -> Feed {
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
