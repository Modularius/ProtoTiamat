use std::collections::HashMap;

use rand::seq::IteratorRandom;

use crate::{structs::{libertee::{Group, Member}, GroupData, LoginAuth, Session, User, UserData}, Uuid};

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
        let mut users = (0..rand::random_range(2..6)).map(|i|(format!("{i}"), User::new(UserData::new_random()))).collect::<HashMap<_,_>>();
        let mut groups = (0..rand::random_range(2..6)).map(|i|(format!("{i}"), Group::new(GroupData::new_random()))).collect::<HashMap<_,_>>();
        
        let num_users = users.len();
        for (user_id, user) in users.iter_mut() {
            user.data.friends = (0..num_users)
                .filter(|_|rand::random_bool(0.5))
                .map(|i|format!("{i}"))
                .collect();
            user.data.groups = (0..groups.len())
                .filter(|_|rand::random_bool(0.5))
                .map(|i|format!("{i}"))
                .collect();
            for (i,group_id) in user.data.groups.iter().enumerate() {
                groups.get_mut(group_id)
                    .unwrap()
                    .data
                    .members
                    .insert(format!("{i}").clone(), Member::new(user_id.clone()));
            }
        }

        let sessions = [(LoginAuth::default(), Session::new(format!("0"), users.get(&format!("0")).unwrap().data.clone(), ))].into_iter().collect::<HashMap<_,_>>();
        Self {
            users,
            groups,
            sessions
        }
    }
}