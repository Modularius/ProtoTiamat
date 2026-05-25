use crate::{impl_error, impl_id, server::{Id, user::User}};
use libertee::{
    Timestamp, id_of,
    traits::{
        HasError, HasId, IsGroup, IsGroupDelegate, IsGroupMember, IsGroupMemberData
    },
};
use strum::Display;
use thiserror::Error;

pub(crate) struct Group {
    id: id_of!(Self),
    members: Vec<GroupMember>,
}
impl_id!(Group, Id);
impl_error!(Group, GroupError);

#[derive(Debug, Error, Display)]
pub(crate) enum GroupError {
    CannotFindUser,
    CannotFindMember,
}

impl IsGroup for Group {
    type UserId = id_of!(User);
    type Member = GroupMember;

    fn get_members(&self) -> Result<&[Self::Member], Self::Error> {
        todo!()
    }

    fn get_member_id_from_user_id(
        &self,
        user_id: &Self::UserId,
    ) -> Result<Option<id_of!(Self::Member)>, Self::Error> {
        Ok(self.members.iter().find_map(|member| {
            (member.get_user_id().expect("") == *user_id).then(|| member.get_id())
        }))
    }

    fn get_member(&self, member_id: &id_of!(Self::Member)) -> Result<&Self::Member, Self::Error> {
        self.members
            .iter()
            .find(|member| member.get_id() == *member_id)
            .ok_or(GroupError::CannotFindMember)
    }

    fn get_member_mut(
        &mut self,
        member_id: &id_of!(Self::Member),
    ) -> Result<&mut Self::Member, Self::Error> {
        self.members
            .iter_mut()
            .find(|member| member.get_id() == *member_id)
            .ok_or(GroupError::CannotFindMember)
    }
}

pub(crate) struct GroupMember {
    user_id: id_of!(User),
    id: id_of!(Self),
}
impl_id!(GroupMember, Id);
impl_error!(GroupMember, GroupMemberError);

#[derive(Debug, Error, Display)]
pub(crate) enum GroupMemberError {
    Misc
}

impl IsGroupMember for GroupMember {
    type UserId = id_of!(User);
    type MemberData = GroupMemberData;
    type Delegate = GroupDelegate;

    fn get_user_id(&self) -> Result<Self::UserId, Self::Error> {
        Ok(self.user_id.clone())
    }
}

pub(crate) struct GroupMemberData {
    influence: f64,
    date_of_joining: Timestamp,
}
impl_error!(GroupMemberData, GroupMemberDataError);

#[derive(Debug, Error, Display)]
pub(crate) enum GroupMemberDataError {
    Misc
}

impl IsGroupMemberData for GroupMemberData {
    fn get_influence(&self) -> Result<f64, Self::Error> {
        Ok(self.influence)
    }

    fn get_date_of_joining(&self) -> Result<Timestamp, Self::Error> {
        Ok(self.date_of_joining)
    }
}

pub(crate) struct GroupDelegate {
    id: id_of!(Self),
}
impl_id!(GroupDelegate, Id);
impl_error!(GroupDelegate, GroupDelegateError);

#[derive(Debug, Error, Display)]
pub(crate) enum GroupDelegateError {
    Misc
}

impl IsGroupDelegate for GroupDelegate {}
