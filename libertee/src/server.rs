use std::collections::HashMap;

use chrono::Utc;
//use elasticsearch::{Elasticsearch, http::transport::Transport};
use elasticsearch as _;
use itertools::Itertools;
use tracing::instrument;

use crate::{
    Group, GroupData, GroupUuid, LiberteeError, LoginAuth, Post, PostUuid, RandomGeneration,
    Session, SessionUuid, Timestamp, User, UserData, UserUuid, Uuid, Uuidlike, user::Friendship,
};

#[derive(Default, Clone, Debug)]
pub struct Server {
    //client: elasticsearch::Elasticsearch,
    users: HashMap<UserUuid, User>,
    groups: HashMap<GroupUuid, Group>,
    sessions: HashMap<SessionUuid, Session>,
    credentials: HashMap<LoginAuth, UserUuid>,
}

impl Server {
    pub fn get_user(&self, uuid: &UserUuid) -> Result<&User, LiberteeError> {
        self.users
            .get(uuid)
            .ok_or_else(|| LiberteeError::NoUserFound(uuid.clone()))
    }

    pub fn get_user_mut(&mut self, uuid: &UserUuid) -> Result<&mut User, LiberteeError> {
        self.users
            .get_mut(uuid)
            .ok_or_else(|| LiberteeError::NoUserFound(uuid.clone()))
    }

    pub fn get_group(&self, uuid: &GroupUuid) -> Result<&Group, LiberteeError> {
        self.groups
            .get(uuid)
            .ok_or_else(|| LiberteeError::NoGroupFound(uuid.clone()))
    }

    pub fn get_group_mut(&mut self, uuid: &GroupUuid) -> Result<&mut Group, LiberteeError> {
        self.groups
            .get_mut(uuid)
            .ok_or_else(|| LiberteeError::NoGroupFound(uuid.clone()))
    }

    #[instrument(skip_all, err)]
    pub fn get_session(&self, uuid: &SessionUuid) -> Result<&Session, LiberteeError> {
        self.sessions
            .get(uuid)
            .ok_or_else(|| LiberteeError::NoSessionFound(uuid.clone()))
    }

    pub fn remove_session(&mut self, uuid: &SessionUuid) -> Option<Session> {
        self.sessions.remove(uuid)
    }

    #[instrument(skip_all, fields(auth, name, datetime))]
    pub fn create_new_user(
        &mut self,
        auth: &LoginAuth,
        name: String,
        datetime: Option<Timestamp>,
    ) -> Result<&mut User, LiberteeError> {
        let user_id = UserUuid(Uuid::generate_random(16));
        self.users.insert(
            user_id.clone(),
            User::new(UserData {
                id: user_id.clone(),
                name,
                datetime_joined: datetime.unwrap_or(Utc::now()),
                ..Default::default()
            }),
        );
        self.credentials.insert(auth.clone(), user_id.clone());
        self.get_user_mut(&user_id)
    }

    #[instrument(skip_all, fields(auth))]
    pub fn create_new_session(&mut self, auth: &LoginAuth) -> Result<&Session, LiberteeError> {
        // Fixme: should guard against clashes with existing Uuids
        let session_id = SessionUuid(Uuid::generate_random(16));
        let user_id = self
            .credentials
            .get(auth)
            .ok_or_else(|| LiberteeError::NoCredentialsFound(auth.clone()))?;

        if !self.users.contains_key(&user_id) {
            return Err(LiberteeError::CredentialsButNoUserFound {
                auth: auth.clone(),
                user_id: user_id.clone(),
            });
        }

        self.sessions.insert(
            session_id.clone(),
            Session::new(
                session_id.clone(),
                user_id.clone(),
                Default::default(),
                //user.data.clone(),
            ),
        );
        self.get_session(&session_id)
    }
    /*
       async fn save(&self) {
           let response = self.client
               .index(IndexParts::Index("sessions"))
               .body(json!({
                   "id": 1
               }))
               .send()
               .await
               .unwrap();

       }
    */
    #[instrument(skip_all, fields(group_id, user_id))]
    pub fn add_post_to_group(
        &mut self,
        group_id: &GroupUuid,
        user_id: &UserUuid,
        subject: String,
        contents: String,
    ) -> Result<PostUuid, LiberteeError> {
        //let member_id = self.get_group(&group_id).and_then(|group|group.get_member_id_from_user_id(user_id));
        let group = self.get_group_mut(&group_id)?;
        let id = group.store.add_post(user_id.clone(), subject, contents);
        //group.store.get_post_mut(id);
        Ok(id)
    }

    #[instrument(skip_all)]
    pub fn create_initial_user(
        &mut self,
        auth: &LoginAuth,
        name: String,
        datetime: Option<Timestamp>,
    ) -> Result<&mut User, LiberteeError> {
        let datetime = datetime.unwrap_or(Utc::now());
        let friendships = self
            .users
            .keys()
            .cloned()
            .filter(|_| rand::random_bool(0.5))
            .collect_vec();
        let groups = self
            .groups
            .keys()
            .cloned()
            .filter(|_| rand::random_bool(0.5))
            .collect_vec();

        let user_id = self
            .create_new_user(auth, name, Some(datetime))
            .unwrap()
            .data
            .id
            .clone();
        for friend_id in friendships {
            self.make_users_friends(&user_id, &friend_id, datetime);
        }
        for group_id in groups {
            self.make_user_group_member(&user_id, &group_id);
        }

        self.get_user_mut(&user_id)
    }

    #[instrument(skip_all)]
    pub fn make_users_friends(
        &mut self,
        user_id1: &UserUuid,
        user_id2: &UserUuid,
        datetime: Timestamp,
    ) {
        if let Ok(user_1) = self.get_user_mut(user_id1) {
            user_1.add_friendship(Friendship {
                user_id: user_id2.clone(),
                datetime_of_friendship: datetime,
            });
        }

        if let Ok(user_2) = self.get_user_mut(user_id2) {
            user_2.add_friendship(Friendship {
                user_id: user_id1.clone(),
                datetime_of_friendship: datetime,
            });
        }
    }

    #[instrument(skip_all)]
    pub fn make_user_group_member(&mut self, user_id: &UserUuid, group_id: &GroupUuid) {
        // FIXME some prior check for group and user existance
        if let Ok(user) = self.get_user_mut(user_id) {
            user.add_group(group_id.clone());
        }
        if let Ok(group) = self.get_group_mut(group_id) {
            group.add_member(user_id.clone());
        }
    }
}

impl RandomGeneration for Server {
    type Parameter = ();

    #[instrument(skip_all)]
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

        for (user_id, user) in users.iter_mut() {
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

            user.data.groups = Some(
                groups
                    .iter()
                    .filter(|_| rand::random_bool(0.5))
                    .map(|(id, _)| id.clone())
                    .collect(),
            );

            for group_id in user.data.groups.iter().flatten() {
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

        let mut server = Self {
            //client: Elasticsearch::new(Transport::single_node("localhost:9200").unwrap()),
            users,
            groups,
            sessions: Default::default(),
            credentials: Default::default(),
        };

        let user_ids = server
            .users
            .keys()
            .cloned()
            .tuple_combinations()
            .filter(|_| rand::random_bool(0.5))
            .collect::<Vec<_>>();

        let datetime_of_friendship = Utc::now();
        for (id1, id2) in user_ids {
            server.make_users_friends(&id1, &id2, datetime_of_friendship);
        }
        server
    }
}
