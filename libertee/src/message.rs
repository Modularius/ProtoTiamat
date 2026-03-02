use std::{
    borrow::Borrow,
    ops::{Deref, Range},
};

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{RandomGeneration, Real, Timestamp, UserUuid, Uuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageUuid(pub Uuid);

impl Into<MessageUuid> for String {
    fn into(self) -> MessageUuid {
        MessageUuid(self)
    }
}

impl ToString for MessageUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub id: MessageUuid,
    pub author: UserUuid,
    pub recepient: UserUuid,
    pub sent_at: Timestamp,
    pub title: String,
    pub content: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use rand::seq::IndexedRandom;

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

        impl RandomGeneration for MessageData {
            type Parameter = (MessageUuid, UserUuid, UserUuid);

            fn new_random((id, author, recepient): Self::Parameter) -> Self {
                Self {
                    id,
                    author,
                    recepient,
                    sent_at: Utc::now(),
                    title: generate_random_text(1..4, 3..6),
                    content: generate_random_text(4..10, 3..10)
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Message {
            pub data: MessageData,
            pub replies: Vec<Message>,
        }

        impl RandomGeneration for Message {
            type Parameter = (MessageUuid, UserUuid, UserUuid);

            fn new_random(id_author: Self::Parameter) -> Self {
                Self {
                    data: MessageData::new_random(id_author),
                    replies: Default::default(),
                }
            }
        }
    }
}
