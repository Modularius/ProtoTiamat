use std::ops::Range;

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{RandomGeneration, Real, Timestamp, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub id: Uuid,
    pub author: Uuid,
    pub posted_at: Timestamp,
    pub title: String,
    pub content: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::seq::IndexedRandom;
        
        #[derive(Default, Clone, Debug)]
        pub struct Feed {
            pub(crate) posts: Vec<Post>
        }

        impl Feed {
            pub(crate) fn add_post(&mut self, author: String, title: String, content: String) -> Uuid {
                let id = (self.posts
                    .iter()
                    .flat_map(|post| post.data
                        .id
                        .parse::<usize>()
                        .ok()
                    )
                    .max()
                    .unwrap_or_default() + 1
                ).to_string();

                self.posts.push(Post {
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

            pub(crate) fn remove_post(&mut self, id: Uuid) {
                if let Some((i,_)) = self.posts.iter().enumerate().find(|(_,post)|post.data.id == id) {
                    self.posts.remove(i);
                }
            }

            pub(crate) fn get_post_mut(&mut self, id: Uuid) -> Option<&mut Post> {
                self.posts.iter_mut().find(|post|post.data.id == id)
            }
        }

        fn generate_random_text(num_words: Range<usize>, word_length: Range<usize>) -> String {
            let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
            (0..rand::random_range(num_words)).map(|_|
                (0..rand::random_range(word_length.clone())).map(|_|
                    alphabet.choose(&mut rand::rng())
                        .to_owned()
                        .unwrap()
                ).collect::<String>()
            )
            .collect::<Vec<_>>()
            .join(" ")
        }

        impl RandomGeneration for PostData {
            type Parameter = (Uuid, Uuid);

            fn new_random((id, author): Self::Parameter) -> Self {
                Self {
                    id,
                    author,
                    posted_at: Utc::now(),
                    title: generate_random_text(1..4, 3..6),
                    content: generate_random_text(4..10, 3..10)
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Post {
            pub(crate) data: PostData,
            pub(crate) replies: Vec<Post>,
            pub(crate) promotions: Real,
        }

        impl RandomGeneration for Post {
            type Parameter = (Uuid, Uuid);

            fn new_random(id_author: Self::Parameter) -> Self {
                Self {
                    data: PostData::new_random(id_author),
                    replies: Default::default(),
                    promotions: 0.0
                }
            }
        }
    }
}
