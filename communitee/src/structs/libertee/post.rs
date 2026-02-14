use std::{borrow::Borrow, ops::{Deref, Range}};

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{RandomGeneration, Real, Timestamp, Uuid, structs::libertee::UserUuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PostUuid(pub Uuid);

impl Into<PostUuid> for String {
    fn into(self) -> PostUuid {
        PostUuid(self)
    }
}

impl ToString for PostUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData {
    pub id: PostUuid,
    pub author: UserUuid,
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
            type Parameter = (PostUuid, UserUuid);

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
            type Parameter = (PostUuid, UserUuid);

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
