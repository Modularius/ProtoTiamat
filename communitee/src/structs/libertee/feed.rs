use std::ops::Range;

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{RandomGeneration, Real, Timestamp, Uuid, structs::Post};

#[derive(Default, Clone, Debug)]
pub struct Feed {
    pub(crate) posts: Vec<Post>
}