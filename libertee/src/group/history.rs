use crate::Timestamp;

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
