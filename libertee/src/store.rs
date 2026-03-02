use crate::{Message, MessageData, MessageUuid, Post, PostUuid, UserUuid, post::PostData};
use chrono::Utc;
use std::collections::BTreeMap;

#[derive(Default, Clone, Debug)]
pub struct Store {
    pub posts: BTreeMap<PostUuid, Post>,
}

impl Store {
    pub fn add_post(&mut self, author: UserUuid, title: String, content: String) -> PostUuid {
        let id = PostUuid(
            (self
                .posts
                .keys()
                .flat_map(|uuid| uuid.0.parse::<usize>().ok())
                .max()
                .unwrap_or_default()
                + 1)
            .to_string(),
        );

        self.posts.insert(
            id.clone(),
            Post {
                data: PostData {
                    id: id.clone(),
                    author,
                    posted_at: Utc::now(),
                    title,
                    content,
                },
                replies: Default::default(),
                promotions: 0.0,
            },
        );
        id
    }

    pub fn remove_post(&mut self, id: PostUuid) {
        self.posts.remove(&id);
    }

    pub fn get_post(&self, id: &PostUuid) -> Option<&Post> {
        self.posts.get(id)
    }

    pub fn get_post_mut(&mut self, id: &PostUuid) -> Option<&mut Post> {
        self.posts.get_mut(id)
    }
}


#[derive(Default, Clone, Debug)]
pub struct MessageStore {
    pub messages: BTreeMap<MessageUuid, Message>,
}

impl MessageStore {
    pub fn add_message(&mut self, author: UserUuid, recepient: UserUuid, title: String, content: String) -> MessageUuid {
        let id = MessageUuid(
            (self
                .messages
                .keys()
                .flat_map(|uuid| uuid.0.parse::<usize>().ok())
                .max()
                .unwrap_or_default()
                + 1)
            .to_string(),
        );

        self.messages.insert(
            id.clone(),
            Message {
                data: MessageData {
                    id: id.clone(),
                    author,
                    recepient,
                    sent_at: Utc::now(),
                    title,
                    content,
                },
                replies: Default::default()
            },
        );
        id
    }

    pub fn remove_post(&mut self, id: MessageUuid) {
        self.messages.remove(&id);
    }

    pub fn get_post(&self, id: &MessageUuid) -> Option<&Message> {
        self.messages.get(id)
    }

    pub fn get_post_mut(&mut self, id: &MessageUuid) -> Option<&mut Message> {
        self.messages.get_mut(id)
    }
}
