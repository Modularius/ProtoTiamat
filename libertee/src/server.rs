use std::collections::HashMap;

use chrono::Utc;

use crate::{
    Group, GroupData, GroupUuid, LoginAuth, Post, PostUuid, RandomGeneration, Session, SessionUuid,
    User, UserData, UserUuid, Uuid, Uuidlike, user::Friendship,
};

#[derive(Default, Clone, Debug)]
pub struct Server {
    users: HashMap<UserUuid, User>,
    groups: HashMap<GroupUuid, Group>,
    sessions: HashMap<SessionUuid, Session>,
    credentials: HashMap<LoginAuth, UserUuid>,
}

impl Server {
    pub fn get_user(&self, uuid: &UserUuid) -> Option<&User> {
        self.users.get(uuid)
    }

    pub fn get_user_mut(&mut self, uuid: &UserUuid) -> Option<&mut User> {
        self.users.get_mut(uuid)
    }

    pub fn get_group(&self, uuid: &GroupUuid) -> Option<&Group> {
        self.groups.get(uuid)
    }

    pub fn get_group_mut(&mut self, uuid: &GroupUuid) -> Option<&mut Group> {
        self.groups.get_mut(uuid)
    }

    pub fn get_session(&self, uuid: &SessionUuid) -> Option<&Session> {
        self.sessions.get(uuid)
    }

    pub fn create_new_session(&mut self, auth: &LoginAuth) -> Option<&Session> {
        // Fixme: should guard against clashes with existing Uuids
        let session_id = SessionUuid(Uuid::generate_random(16));
        if let Some(user_id) = self.credentials.get(auth) {
            if let Some(user) = self.get_user(user_id) {
                self.sessions.insert(
                    session_id.clone(),
                    Session::new(session_id.clone(), user_id.clone(), user.data.clone()),
                );
            }
        }
        self.sessions.get(&session_id)
    }
}

impl RandomGeneration for Server {
    type Parameter = ();

    fn new_random(_: Self::Parameter) -> Self {
        let mut users = (0..rand::random_range(14..19))
            .map(|i| {
                let id = UserUuid(i.to_string());
                (id.clone(), User::new(UserData::new_random(id)))
            })
            .collect::<HashMap<_, _>>();

        let mut groups = (0..rand::random_range(5..8))
            .map(|i| {
                let id = GroupUuid(i.to_string());
                (id.clone(), Group::new(GroupData::new_random(id)))
            })
            .collect::<HashMap<_, _>>();

        let user_ids = users.keys().cloned().collect::<Vec<_>>();
        for (user_id, user) in users.iter_mut() {
            user.data.friends = user_ids
                .iter()
                .filter(|_| rand::random_bool(0.5))
                .filter(|&id| id != user_id)
                .map(|id| Friendship {
                    user_id: id.clone(),
                    datetime_of_friendship: Utc::now(),
                })
                .collect();

            user.data.groups = groups
                .iter()
                .filter(|_| rand::random_bool(0.5))
                .map(|(id, _)| id.clone())
                .collect();

            user.store.posts = (0..rand::random_range(6..11))
                .map(|id| {
                    let mut post = Post::new_random((PostUuid(id.to_string()), user_id.clone()));
                    post.replies = (0..rand::random_range(0..2))
                        .map(|rid| {
                            Post::new_random((
                                PostUuid((rid + id * 10000).to_string()),
                                user_id.clone(),
                            ))
                        })
                        .collect::<Vec<_>>();
                    (post.data.id.clone(), post)
                })
                .collect();

            for group_id in user.data.groups.iter() {
                let group = groups.get_mut(group_id).unwrap();
                group.add_member(user_id.clone());
                for _ in 0..4 {
                    let post = Post::new_random((
                        PostUuid(group.store.posts.len().to_string()),
                        user_id.clone(),
                    ));
                    group
                        .store
                        .add_post(post.data.author, post.data.title, post.data.content);
                }
            }
        }

        let credentials = [(LoginAuth::default(), UserUuid("0".into()))]
            .into_iter()
            .collect::<HashMap<_, _>>();
        Self {
            users,
            groups,
            sessions: Default::default(),
            credentials,
        }
    }
}
