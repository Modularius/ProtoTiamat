use crate::traits::{HasError, HasId, IsGroup, IsGroupDelegate, IsGroupMember, IsId, IsServer, IsUser};

/// Implements the top-level api that the client uses to interact with the server.
/// 
pub trait IsClientInterface : HasError {
    type Server: IsServer<User = Self::User, Group = Self::Group>;
    type User : IsUser;
    type Group : IsGroup;
    
    fn get_this_user_data(&self) -> Result<<Self::User as IsUser>::UserData, Self::Error>;
    fn get_other_user_data(&self, user_id: &<Self::User as HasId>::Id) -> Result<<Self::User as IsUser>::UserData, Self::Error>;
}

pub trait IsClientGroupInterface : HasError {
    type Server: IsServer<Group = Self::Group, User = Self::User>;
    type User : IsUser;
    type Group : IsGroup<Member = Self::Member>;
    type Member : IsGroupMember;

    fn join_group(&self, id: &<Self::Group as HasId>::Id) -> Result<(), Self::Error>;
    fn leave_group(&self, id: &<Self::Group as HasId>::Id) -> Result<(), Self::Error>;
    
    fn get_user_member_id(&self, id: &<Self::User as HasId>::Id) -> Result<Option<<Self::Member as HasId>::Id>, Self::Error>;

    fn get_member_data(&self, id: &<Self::Member as HasId>::Id) -> Result<<Self::Member as IsGroupMember>::MemberData, Self::Error>;
}

pub trait IsClientGroupMemberInterface : HasError + IsClientBoardInterface<BoardId = <Self::Group as HasId>::Id> {
    type Server: IsServer<Group = Self::Group>;
    type Group : IsGroup<Member = Self::Member>;
    type Member : IsGroupMember<Delegate = Self::Delegate>;
    type Delegate : IsGroupDelegate;

    fn get_delegates(&self) -> Result<&[Self::Delegate], Self::Error>;
    fn make_member_delegate(&self, id: &<Self::Member as HasId>::Id) -> Result<Self::Delegate, Self::Error>;
    fn remove_delegate(&self, id: &<Self::Delegate as HasId>::Id) -> Result<(), Self::Error>;
}

pub trait IsClientGroupDelegateInterface : HasError {
    fn set_weight(&mut self, weight: f64) -> Result<(), Self::Error>;
    fn get_weight(&self) -> Result<f64, Self::Error>;
}

pub trait IsClientGroupAdminInterface : HasError {
    type Server: IsServer<Group = Self::Group>;
    type Group : IsGroup;

    fn rename_group(&self, id: &<Self::Group as HasId>::Id) -> Result<(), Self::Error>;
    fn delete_group(&self, id: &<Self::Group as HasId>::Id) -> Result<(), Self::Error>;
}

pub trait IsClientBoardInterface : HasError {
    type PostId : IsId;
    type BoardId : IsId;
    
    fn generate_feed(&self) -> Result<(), Self::Error>;
    fn continue_feed(&self) -> Result<(), Self::Error>;

    fn make_new_post(&self, id: &Self::BoardId) -> Result<(), Self::Error>;
    fn delete_post(&self, id: &Self::BoardId, post_id: &Self::PostId) -> Result<(), Self::Error>;

    fn reply_to_post(&self, id: &Self::BoardId, post_id: &Self::PostId) -> Result<(), Self::Error>;
    fn private_reply_to_post(&self, id: &Self::BoardId, post_id: &Self::PostId) -> Result<(), Self::Error>;

    fn promote_post(&self, id: &Self::BoardId, post_id: &Self::PostId) -> Result<(), Self::Error>;
    fn depromote_post(&self, id: &Self::BoardId, post_id: &Self::PostId) -> Result<(), Self::Error>;
}