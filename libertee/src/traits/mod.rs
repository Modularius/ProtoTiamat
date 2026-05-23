mod client_interfaces;

use std::fmt::Display;

use crate::Timestamp;

pub trait IsServer : HasError {
    type User: IsUser;
    type Group: IsGroup<Member = Self::GroupMember>;
    type GroupMember: IsGroupMember;

    fn find_user(&self, user_id: &<Self::User as HasId>::Id) -> Result<(), Self::Error>;
    fn find_group(&self, group_id: &<Self::Group as HasId>::Id) -> Result<(), Self::Error>;

    fn get_group_member_id_from_user_id(&self, user_id: &<Self::User as HasId>::Id, group_id: &<Self::Group as HasId>::Id) -> Result<Option<<Self::GroupMember as HasId>::Id>, Self::Error>;
}

pub trait IsId {
    type Error;
}

pub trait HasId {
    type Id: IsId;

    fn get_id(&self) -> Self::Id;
}

pub trait HasError {
    type Error;
}

pub trait IsUser : HasId + HasError {
    type UserData: IsUserData;

    fn get_friend_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_friend(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;

    fn get_follow_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_followed(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;

    fn get_blocked_list(&self) -> Result<Vec<Self::Id>, Self::Error>;
    fn is_user_blocked(&self, user_id: &Self::Id) -> Result<bool, Self::Error>;
}

pub trait IsUserData: HasError {
    fn get_name(&self) -> Result<String, Self::Error>;
    fn set_name(&mut self, new_name: &str) -> Result<(), Self::Error>;
}

pub trait IsGroup : HasId + HasError {
    type UserData: IsUserData;
    type Member: IsGroupMember;

    fn get_members(&self) -> Result<&[Self::Member], Self::Error>;
    fn get_member_from_user(&self, user_id: &Self::Id) -> Result<Self::Member, Self::Error>;
}

pub trait IsBoard : HasId + HasError {
    type Post: IsPost;

    fn get_post(&self, id: &<Self::Post as HasId>::Id) -> Result<Self::Post, Self::Error>;

    fn new_post(&self, id: &<Self::Post as HasId>::Id) -> Result<Self::Post, Self::Error>;
}

pub trait IsGroupMember : HasId + HasError {
    type MemberData: IsGroupMemberData;
    type Delegate : IsGroupDelegate;
}

pub trait IsGroupMemberData : HasError {
    fn get_influence(&self) -> Result<f64, Self::Error>;
    fn get_date_of_joining(&self) -> Result<Timestamp, Self::Error>;
}

pub trait IsGroupDelegate : HasId + HasError {
}

pub trait IsPost : HasId + HasError + Sized {
    type User: IsUser;
    type Content: Display;

    fn get_author(&self) -> Result<<Self::User as HasId>::Id, Self::Error>;
    fn get_title(&self) -> Result<String, Self::Error>;
    fn get_date_of_post(&self) -> Result<Self::Content, Self::Error>;
    fn get_content(&self) -> Result<Self::Content, Self::Error>;
    fn get_replies(&self) -> Result<&[Self], Self::Error>;

    fn get_popularity(&self) -> Result<f64, Self::Error>;
}