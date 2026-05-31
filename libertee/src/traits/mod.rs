mod admin_interface;
mod client_interfaces;

use std::{error::Error, fmt::Display};

pub use admin_interface::IsAdminInterface;
pub use client_interfaces::{IsLoginCred, IsClientInterface};

use crate::Timestamp;

#[macro_export]
macro_rules! cast {
    ($type:ty as $trait:ty : $sub:ident) => {
        <$type as $trait>::$sub
    };
}

#[macro_export]
macro_rules! id_of {
    ($type:ty) => {
        <$type as HasId>::Id
    };
}

#[macro_export]
macro_rules! err_of {
    ($type:ident) => {
        <$type as HasError>::Error
    };
}

pub trait IsServer: HasError {
    type User: IsUser;
    type Group: IsGroup<Member = Self::GroupMember>;
    type GroupMember: IsGroupMember<UserId = id_of!(Self::User)>;
    type LoginCred;

    fn find_user(&self, user_id: &id_of!(Self::User)) -> Result<&Self::User, Self::Error>;
    fn find_group(
        &self,
        group_id: &id_of!(Self::Group),
    ) -> Result<&Self::Group, Self::Error>;

    fn find_user_mut(&mut self, user_id: &id_of!(Self::User)) -> Result<&mut Self::User, Self::Error>;
    fn find_group_mut(
        &mut self,
        group_id: &id_of!(Self::Group),
    ) -> Result<&mut Self::Group, Self::Error>;

    fn get_group_member_id_from_user_id(
        &self,
        user_id: &id_of!(Self::User),
        group_id: &id_of!(Self::Group),
    ) -> Result<Option<id_of!(Self::GroupMember)>, Self::Error>;

    fn login(&self, login: Self::LoginCred) -> Result<id_of!(Self::User), Self::Error>;
}

pub trait IsId {
    type Error;
}

pub trait HasId {
    type Id: IsId;

    fn get_id(&self) -> Self::Id;
}

pub trait HasError {
    type Error : Error;
}

pub trait IsUser: HasId + HasError {
    type UserData: IsUserData;
    type EncryptedPassword;

    fn get_data(&self) -> Result<&Self::UserData, Self::Error>;
    
    fn get_groups_list(&self) -> Result<Vec<id_of!(Self::Group)>, Self::Error>;
    fn is_in_group(&self, group_id: &Self::Id) -> Result<bool, Self::Error>;

    fn get_friend_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_friend(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;

    fn get_follow_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_followed(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;

    fn get_blocked_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_blocked(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;

    fn is_password(&self, password: Self::EncryptedPassword) -> Result<bool, Self::Error>;
}

pub trait IsUserData: HasError + Clone {
    fn get_name(&self) -> Result<String, Self::Error>;
    fn set_name(&mut self, new_name: &str) -> Result<(), Self::Error>;
    fn get_friends(&self, new_name: &str) -> Result<(), Self::Error>;
}

pub trait IsGroup: HasId + HasError {
    type UserId: IsId;
    type Member: IsGroupMember<UserId = Self::UserId>;

    fn get_members(&self) -> Result<&[Self::Member], Self::Error>;

    fn get_member_id_from_user_id(
        &self,
        user_id: &Self::UserId,
    ) -> Result<Option<id_of!(Self::Member)>, Self::Error>;
    fn get_member(&self, member_id: &id_of!(Self::Member)) -> Result<&Self::Member, Self::Error>;
    fn get_member_mut(
        &mut self,
        member_id: &id_of!(Self::Member),
    ) -> Result<&mut Self::Member, Self::Error>;
}

pub trait IsBoard: HasId + HasError {
    type Post: IsPost;

    fn get_post(&self, id: &id_of!(Self::Post)) -> Result<Self::Post, Self::Error>;

    fn new_post(&self, id: &id_of!(Self::Post)) -> Result<Self::Post, Self::Error>;
}

pub trait IsGroupMember: HasId + HasError {
    type UserId: IsId;
    type MemberData: IsGroupMemberData;
    type Delegate: IsGroupDelegate;

    fn get_user_id(&self) -> Result<Self::UserId, Self::Error>;
}

pub trait IsGroupMemberData: HasError {
    fn get_influence(&self) -> Result<f64, Self::Error>;
    fn get_date_of_joining(&self) -> Result<Timestamp, Self::Error>;
}

pub trait IsGroupDelegate: HasId + HasError {}

pub trait IsPost: HasId + HasError + Sized {
    type User: IsUser;
    type Content: Display;

    fn get_author(&self) -> Result<id_of!(Self::User), Self::Error>;
    fn get_title(&self) -> Result<String, Self::Error>;
    fn get_date_of_post(&self) -> Result<Self::Content, Self::Error>;
    fn get_content(&self) -> Result<Self::Content, Self::Error>;
    fn get_replies(&self) -> Result<&[Self], Self::Error>;

    fn get_popularity(&self) -> Result<f64, Self::Error>;
}
