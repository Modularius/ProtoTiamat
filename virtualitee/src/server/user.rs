use crate::{impl_error, impl_id, server::{Id, group::Group}};
use libertee::{
    id_of,
    traits::{
        HasError, HasId, IsUser, IsUserData,
    },
};
use strum::Display;
use thiserror::Error;

pub(crate) struct User {
    /// Unique identifier of user.
    id: id_of!(Self),
    
    /// List of all users the user is friends with.
    friend_list: Vec<id_of!(Self)>,
    
    /// List of all users the user has blocked.
    blocked_list: Vec<id_of!(Self)>,

    /// List of all users the user follows.
    follow_list: Vec<id_of!(Self)>,

    /// The user's data field.
    data: UserData,

    /// This is the super secret unencrypted password.
    password: String,

    /// Groups that the user is a member of.
    group_list: Vec<id_of!(Group)>,
}

impl_id!(User, Id);
impl_error!(User, UserError);

#[derive(Debug, Error, Display)]
pub(crate) enum UserError {
    Misc
}

#[derive(Clone)]
pub(crate) struct UserData {
    name: String,
    friends: Vec<id_of!(User)>,
}
impl_error!(UserData, UserDataError);

#[derive(Debug, Error, Display)]
pub(crate) enum UserDataError {
    Misc
}

impl IsUserData for UserData {
    fn get_name(&self) -> Result<String, Self::Error> {
        Ok(self.name.clone())
    }

    fn set_name(&mut self, new_name: &str) -> Result<(), Self::Error> {
        self.name = new_name.to_string();
        Ok(())
    }
}

impl IsUser for User {
    type UserData = UserData;
    type EncryptedPassword = String;

    fn get_friend_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        Ok(self.friend_list.clone())
    }

    fn is_user_friend(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.friend_list.contains(user_id))
    }

    fn get_follow_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        Ok(self.follow_list.clone())
    }

    fn is_user_followed(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.follow_list.contains(user_id))
    }

    fn get_blocked_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        Ok(self.blocked_list.clone())
    }

    fn is_user_blocked(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.blocked_list.contains(user_id))
    }
    
    fn get_data(&self) -> Result<&Self::UserData, Self::Error> {
        Ok(&self.data)
    }
    
    fn is_password(&self, password: Self::EncryptedPassword) -> Result<bool, Self::Error> {
        Ok(self.password == password)
    }
}
