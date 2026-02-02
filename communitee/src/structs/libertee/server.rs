use std::collections::HashMap;

use crate::{
    Uuid,
    structs::{
        GroupData, LoginAuth, Session, User, UserData,
        libertee::{Group, Member, Post},
    },
};

#[derive(Default, Clone, Debug)]
pub struct Server {
    users: HashMap<Uuid, User>,
    groups: HashMap<Uuid, Group>,
    sessions: HashMap<LoginAuth, Session>,
}

impl Server {
    pub(crate) fn get_user(&self, uuid: Uuid) -> Option<&User> {
        self.users.get(&uuid)
    }

    pub(crate) fn get_user_mut(&mut self, uuid: Uuid) -> Option<&mut User> {
        self.users.get_mut(&uuid)
    }

    pub(crate) fn get_group(&self, uuid: Uuid) -> Option<&Group> {
        self.groups.get(&uuid)
    }

    pub(crate) fn get_group_mut(&mut self, uuid: Uuid) -> Option<&mut Group> {
        self.groups.get_mut(&uuid)
    }

    pub(crate) fn get_session(&self, auth: &LoginAuth) -> Option<&Session> {
        self.sessions.get(auth)
    }

    pub fn new_random() -> Self {
        let mut users = (0..rand::random_range(2..6))
            .map(|i| (format!("{i}"), User::new(UserData::new_random(format!("{i}")))))
            .collect::<HashMap<_, _>>();

        let mut groups = (0..rand::random_range(2..6))
            .map(|i| (format!("{i}"), Group::new(GroupData::new_random(format!("{i}")))))
            .collect::<HashMap<_, _>>();

        let num_users = users.len();
        for (user_id, user) in users.iter_mut() {
            user.data.friends = (0..num_users)
                .filter(|_| rand::random_bool(0.5))
                .map(|i| format!("{i}"))
                .collect();

            user.data.groups = (0..groups.len())
                .filter(|_| rand::random_bool(0.5))
                .map(|i| format!("{i}"))
                .collect();

            user.feed.posts = (0..rand::random_range(3..6))
                .map(|id| Post::new_random(format!("{id}"), user_id.clone()))
                .collect();

            for group_id in user.data.groups.iter() {
                let group = groups.get_mut(group_id).unwrap();
                let member_id = format!("{}", group.data.members.len());
                let post_id = format!("{}", group.feed.posts.len());
                group.data
                    .members
                    .insert(member_id.clone(), Member::new(member_id, user_id.clone()));
                group.feed.posts.push(Post::new_random(post_id, user_id.clone()))
            }
        }

        let sessions = [(
            LoginAuth::default(),
            Session::new(format!("0"), users.get(&format!("0")).unwrap().data.clone()),
        )]
        .into_iter()
        .collect::<HashMap<_, _>>();
        Self {
            users,
            groups,
            sessions,
        }
    }
}
