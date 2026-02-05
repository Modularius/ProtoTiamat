use std::collections::HashMap;

use chrono::Utc;

use crate::{
    Uuid,
    structs::{
        GroupData, LoginAuth, Session, User, UserData,
        libertee::{Group, Post, user::Friendship},
    },
};

#[derive(Default, Clone, Debug)]
pub struct Server {
    users: HashMap<Uuid, User>,
    groups: HashMap<Uuid, Group>,
    sessions: HashMap<LoginAuth, Session>,
}

impl Server {
    pub(crate) fn get_user(&self, uuid: &Uuid) -> Option<&User> {
        self.users.get(uuid)
    }

    pub(crate) fn get_user_mut(&mut self, uuid: &Uuid) -> Option<&mut User> {
        self.users.get_mut(uuid)
    }

    pub(crate) fn get_group(&self, uuid: &Uuid) -> Option<&Group> {
        self.groups.get(uuid)
    }

    pub(crate) fn get_group_mut(&mut self, uuid: &Uuid) -> Option<&mut Group> {
        self.groups.get_mut(uuid)
    }

    pub(crate) fn get_session(&self, auth: &LoginAuth) -> Option<&Session> {
        self.sessions.get(auth)
    }

    pub fn new_random() -> Self {
        let mut users = (0..rand::random_range(14..19))
            .map(|i| (i.to_string(), User::new(UserData::new_random(i.to_string()))))
            .collect::<HashMap<_, _>>();

        let mut groups = (0..rand::random_range(5..8))
            .map(|i| (i.to_string(), Group::new(GroupData::new_random(i.to_string()))))
            .collect::<HashMap<_, _>>();

        let user_ids = users.keys().cloned().collect::<Vec<_>>();
        for (user_id, user) in users.iter_mut() {
            user.data.friends = user_ids.iter()
                .filter(|_| rand::random_bool(0.5))
                .filter(|&id| id != user_id)
                .map(|id| Friendship {
                    user_id: id.clone(),
                    datetime_of_friendship: Utc::now(),
                })
                .collect();

            user.data.groups = groups.iter()
                .filter(|_| rand::random_bool(0.5))
                .map(|(id, _)| id.clone())
                .collect();

            user.feed.posts = (0..rand::random_range(6..11))
                .map(|id| Post::new_random(id.to_string(), user_id.clone()))
                .collect();

            for group_id in user.data.groups.iter() {
                let group = groups.get_mut(group_id).unwrap();
                group.add_member(user_id.clone());
                for _ in 0..4 {
                    group.feed
                        .posts
                        .push(Post::new_random(
                            group.feed
                                .posts
                                .len()
                                .to_string(),
                            user_id.clone()
                        ))
                }
            }
        }

        let sessions = [(
            LoginAuth::default(),
            Session::new("0".into(), users.get(&"0".to_string())
                .unwrap()
                .data
                .clone()
            ),
        )].into_iter()
            .collect::<HashMap<_, _>>();
        Self {
            users,
            groups,
            sessions,
        }
    }
}
