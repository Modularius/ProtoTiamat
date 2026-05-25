use crate::{
    id_of,
    traits::{HasError, HasId, IsGroup, IsGroupDelegate, IsGroupMember, IsId, IsServer, IsUser},
};

/// Implements the top-level api that the client uses to interact with the server.
///
pub trait IsAdminInterface<'a>: HasError where <Self as IsAdminInterface<'a>>::User : 'a  {
    type Server: IsServer<User = Self::User, Group = Self::Group>;
    type User: IsUser;
    type Group: IsGroup;
    type UserIterator: Iterator<Item = &'a Self::User>;

    fn iter_user(&self) -> Result<Self::UserIterator, Self::Error>;
    fn iter_group(&self) -> Result<Self::UserIterator, Self::Error>;
}
