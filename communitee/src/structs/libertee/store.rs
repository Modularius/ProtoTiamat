use std::collections::BTreeMap;
use chrono::Utc;
use crate::{Uuid, structs::{Post, libertee::{PostUuid, UserUuid, post::PostData}}};

#[derive(Default, Clone, Debug)]
pub struct Store {
    pub(crate) posts: BTreeMap<PostUuid, Post>
}

impl Store {
    pub(crate) fn add_post(&mut self, author: UserUuid, title: String, content: String) -> PostUuid {
        let id = PostUuid((self.posts
            .keys()
            .flat_map(|uuid| uuid.0
                .parse::<usize>()
                .ok()
            )
            .max()
            .unwrap_or_default() + 1
        ).to_string());

        self.posts.insert(id.clone(), Post {
            data: PostData {
                id: id.clone(),
                author,
                posted_at: Utc::now(),
                title,
                content
            },
            replies: Default::default(),
            promotions: 0.0
        });
        id
    }

    pub(crate) fn remove_post(&mut self, id: PostUuid) {
        self.posts.remove(&id);
    }

    pub(crate) fn get_post_mut(&mut self, id: PostUuid) -> Option<&mut Post> {
        self.posts.get_mut(&id)
    }
}