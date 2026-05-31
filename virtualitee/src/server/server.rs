use crate::{impl_error, impl_id, server::{client_interface::LoginCredentials, group::{GroupError, GroupMember}, user::UserError}};
use super::{user::User, group::Group};
use libertee::{
    id_of,
    traits::{
        HasError, HasId, IsAdminInterface, IsClientInterface, IsGroup, IsLoginCred, IsServer, IsUser, IsUserData
    },
};
use strum::Display;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub(crate) enum ServerError {
    CannotFindUser,
    CannotFindGroup,
    LoginFailed,
    GroupError(#[from] GroupError),
    UserError(#[from] UserError)
}

#[derive(Default)]
pub(crate) struct Server {
    users: Vec<User>,
    groups: Vec<Group>,
}
impl_error!(Server, ServerError);

impl Server {
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

impl IsServer for Server {
    type User = User;
    type Group = Group;
    type GroupMember = GroupMember;
    type LoginCred = LoginCredentials;
    
    fn login(&self, login: Self::LoginCred) -> Result<id_of!(Self::User), Self::Error> {
        self.users
            .iter()
            .find_map(|user|
                (
                    user.get_data().ok()?.get_name().ok()? == login.get_user_name().ok()?
                    && user.is_password(login.get_password().ok()?.to_owned()).ok()?
                )
                .then_some(user.get_id())
            )
            .ok_or(ServerError::LoginFailed)
    }

    fn find_user(&self, user_id: &id_of!(Self::User)) -> Result<&Self::User, Self::Error> {
        self.users
            .iter()
            .find(|user| user.get_id() == *user_id)
            .ok_or(ServerError::CannotFindUser)
    }

    fn find_group(
        &self,
        group_id: &id_of!(Self::Group),
    ) -> Result<&Self::Group, Self::Error> {
        self.groups
            .iter()
            .find(|group| group.get_id() == *group_id)
            .ok_or(ServerError::CannotFindGroup)
    }

    fn find_user_mut(&mut self, user_id: &id_of!(Self::User)) -> Result<&mut Self::User, Self::Error> {
        self.users
            .iter_mut()
            .find(|user| user.get_id() == *user_id)
            .ok_or(ServerError::CannotFindUser)
    }

    fn find_group_mut(
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


impl<'a> IsAdminInterface<'a> for Server {
    type Server = Self;
    type User = User;
    type Group = Group;
    type UserIterator = std::slice::Iter<'a, User>;

    fn iter_user(&self) -> Result<Self::UserIterator, Self::Error> {
        todo!()
    }

    fn iter_group(&self) -> Result<Self::UserIterator, Self::Error> {
        todo!()
    }
}