use libertee::{id_of, traits::{HasError, HasId, IsGroup, IsGroupMember, IsId, IsServer, IsUser, IsUserData}};

pub(crate) struct Server {
    
}

impl HasError for Server { type Error = String; }

#[derive(Default, Clone, Debug)]
struct Id(String);

impl IsId for Id {
    type Error = String;
}

pub(crate) struct User {
    id: id_of!(Self),
}

#[macro_export]
macro_rules! impl_id {
    ($type:ty, $id_type:ty) => {
        impl HasId for $type {
            type Id = $id_type;
            
            fn get_id(&self) -> Self::Id {
                self.id.clone()
            }
        }
    }
}
impl HasError for User { type Error = String; }
impl_id!(User,Id);

pub(crate) struct UserData {
    name: String,
    friends: Vec<id_of!(User)>,
}
impl HasError for UserData { type Error = String; }
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

    fn get_friend_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        todo!()
    }

    fn is_user_friend(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        todo!()
    }

    fn get_follow_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        todo!()
    }

    fn is_user_followed(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        todo!()
    }

    fn get_blocked_list(&self) -> Result<Vec<Self::Id>, Self::Error> {
        todo!()
    }

    fn is_user_blocked(&self, user_id: &Self::Id) -> Result<bool, Self::Error> {
        todo!()
    }
}

pub(crate) struct Group {
    id: id_of!(Self),
}
impl HasError for Group { type Error = String; }
impl_id!(Group,Id);

impl IsGroup for Group {
    type UserData = UserData;
    type Member = GroupMember;

    fn get_members(&self) -> Result<&[Self::Member], Self::Error> {
        todo!()
    }

    fn get_member_from_user(&self, user_id: &Self::Id) -> Result<Self::Member, Self::Error> {
        todo!()
    }
}

pub(crate) struct GroupMember {
    id: id_of!(Self),
}
impl HasError for GroupMember { type Error = String; }
impl_id!(GroupMember,Id);

impl IsGroupMember for GroupMember {
    type MemberData;

    type Delegate ;
}

impl IsServer for Server {
    type User = User;

    type Group = Group;

    type GroupMember = GroupMember;

    fn find_user(&self, user_id: &<Self::User as libertee::traits::HasId>::Id) -> Result<(), Self::Error> {
        todo!()
    }

    fn find_group(&self, group_id: &<Self::Group as libertee::traits::HasId>::Id) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_group_member_id_from_user_id(&self, user_id: &<Self::User as libertee::traits::HasId>::Id, group_id: &<Self::Group as libertee::traits::HasId>::Id) -> Result<Option<<Self::GroupMember as libertee::traits::HasId>::Id>, Self::Error> {
        todo!()
    }
}