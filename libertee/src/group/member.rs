use cfg_if::cfg_if;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{Real, Timestamp, UserUuid, Uuid};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberUuid(pub Uuid);

impl Into<MemberUuid> for String {
    fn into(self) -> MemberUuid {
        MemberUuid(self)
    }
}

impl ToString for MemberUuid {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Delegate {
    pub delegate_id: MemberUuid,
    pub timestamp: Timestamp,
    pub weight: Real,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: MemberUuid,
    pub user: UserUuid,
    pub joined: Timestamp,
    pub power: Real,
    pub delegates: HashMap<MemberUuid, Delegate>,
    pub delegators: HashSet<MemberUuid>,
}

impl Member {
    pub fn new(id: MemberUuid, user: UserUuid) -> Self {
        Self {
            id,
            user,
            joined: Utc::now(),
            power: Real::default(),
            delegates: Default::default(),
            delegators: Default::default(),
        }
    }
}
cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl Member {
            pub fn delegate_to(&mut self, delegator: &MemberUuid, weight_change: Real) {
                if !self.delegators.contains(delegator) {
                    self.delegators.insert(delegator.clone());
                }
                self.power += weight_change;
            }

            pub fn remove_delegator(&mut self, delegator: &MemberUuid, weight: Real) {
                self.delegators.remove(delegator);
                self.power -= weight;
            }

            pub fn prune_null_delegators(&mut self, members: &BTreeMap<MemberUuid, Member>) {
                let mut remove : Vec<_> = Default::default();
                for delegator in &self.delegators {
                    if let Some(delegator_member) = members.get(delegator) {
                        if delegator_member.delegates.get(&self.id).is_none() {
                            remove.push(delegator.clone())
                        }
                    } else {
                        remove.push(delegator.clone())
                    }
                }
                for delegator in remove {
                    self.delegators.remove(&delegator);
                }
            }

            pub fn calc_power_value(&self, members: &BTreeMap<MemberUuid, Member>) -> Real {
                self.delegators.iter()
                    .flat_map(|delegator| members.get(delegator)
                        .and_then(|delegator_member| delegator_member.delegates
                            .get(&self.id)
                            .map(|d|d.weight)
                        )
                    ).sum()
            }

            pub fn reset_power_value(&mut self, members: &BTreeMap<MemberUuid, Member>) {
                self.power = self.calc_power_value(members);
            }

            pub fn verify_power_value(&self, members: &BTreeMap<MemberUuid, Member>) -> bool {
                let power_value = self.calc_power_value(members);
                power_value == self.power
            }
        }
    }
}