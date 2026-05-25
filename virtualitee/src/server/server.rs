use crate::{impl_error, impl_id, server::group::{GroupError, GroupMember}};
use super::{user::User, group::Group};
use libertee::{
    id_of,
    traits::{
        HasError, HasId, IsGroup,
        IsServer,
    },
};
use strum::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub(crate) enum ServerError {
    CannotFindUser,
    CannotFindGroup,
    GroupError(#[from] GroupError)
}

pub(crate) struct Server {
    users: Vec<User>,
    groups: Vec<Group>,
}
impl_error!(Server, ServerError);

impl IsServer for Server {
    type User = User;
    type Group = Group;
    type GroupMember = GroupMember;

    fn find_user(&mut self, user_id: &id_of!(Self::User)) -> Result<&mut Self::User, Self::Error> {
        self.users
            .iter_mut()
            .find(|user| user.get_id() == *user_id)
            .ok_or(ServerError::CannotFindUser)
    }

    fn find_group(
        &mut self,
        group_id: &id_of!(Self::Group),
    ) -> Result<&mut Self::Group, Self::Error> {
        self.groups
            .iter_mut()
            .find(|group| group.get_id() == *group_id)
            .ok_or(ServerError::CannotFindGroup)
    }

    fn get_group_member_id_from_user_id(
        &self,
        user_id: &id_of!(Self::User),
        group_id: &id_of!(Self::Group),
    ) -> Result<Option<id_of!(Self::GroupMember)>, Self::Error> {
        Ok(self
            .groups
            .iter()
            .find(|group| group.get_id() == *group_id)
            .ok_or(ServerError::CannotFindGroup)?
            .get_member_id_from_user_id(user_id)?)
    }
}
