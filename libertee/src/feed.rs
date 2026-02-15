use std::ops::Range;

use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{Post, RandomGeneration, Real, Timestamp, Uuid};

#[derive(Default, Clone, Debug)]
pub struct Feed {
    pub posts: Vec<Post>,
}
