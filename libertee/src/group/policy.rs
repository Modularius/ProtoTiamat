use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{GroupUuid, RandomGeneration, Real, Timestamp, UserUuid, Uuid, group::MemberUuid};

pub enum PolicyType {
    Rename {
        new_name: String
    },
    ChangeGroupProperty {
        property: String,
        new_value: String,
    },
    MergeWith {
        group_id: GroupUuid,
        coefficient: Real,
    },
    DeleteGroup,
    TagToGroupPolicySet {
        group_id: GroupUuid,
        filter: String,
    },
    AllowAdvert {
        advert_id: String,
        weight: Real,
    },
    BanAdvert {
        advert_id: String,
    },
    RecruitAdmins,
}

pub struct Policy {
    policy_type: PolicyType,
    proposed_reason: String,
    proposer: MemberUuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GroupPermission {
    Delete,
    Rename,
    Merge,
    ProposePolicy,
    SendAnnouncement,
    PostPrivate,
    VoteOnPolicy,
    Moderate,
    PenaliseMember
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupAdmin {
    member_id: MemberUuid,
    permissions: Vec<GroupPermission>
}
