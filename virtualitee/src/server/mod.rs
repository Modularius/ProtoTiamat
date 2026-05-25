mod client_interface;
mod server;
mod group;
mod user;

use libertee::traits::IsId;

pub(crate) use server::Server;
pub(crate) use client_interface::ClientInterface;

#[macro_export]
macro_rules! impl_id {
    ($type:ty, $id_type:ty) => {
        impl HasId for $type {
            type Id = $id_type;

            fn get_id(&self) -> Self::Id {
                self.id.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_error {
    ($type:ty, $err_type:ty) => {
        
        impl HasError for $type {
            type Error = $err_type;
        }
    };
}


#[derive(Default, Clone, Debug, PartialEq)]
pub(crate) struct Id(String);

impl IsId for Id {
    type Error = String;
}
