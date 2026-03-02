use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{RandomGeneration, Real, Timestamp, UserUuid, Uuid};

#[derive(Clone, Debug)]
pub enum GroupPolicyHistory {
    Proposal(String),
    VoteResult(String),
    Adoption(String),
    Removal(String),
}

#[derive(Clone, Debug)]
pub enum GroupHistory {
    Creation(Timestamp),
    Named(String),
    Policy(GroupPolicyHistory),
    Merge(String),
}
