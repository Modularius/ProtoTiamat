use libertee::{id_of, traits::{HasError, HasId, IsClientInterface, IsLoginCred, IsServer, IsUser}};
use strum::Display;
use thiserror::Error;

use crate::{impl_error, server::{Server, group::Group, server::ServerError, user::{User, UserData}}};

pub(crate) struct ClientInterface {
    server: Server,
    login: Option<id_of!(User)>,
}

impl ClientInterface {
    pub(crate) fn new(server: Server) -> Self {
        Self { server, login: None }
    }

    pub(crate) fn get_server_mut(&mut self) -> &mut Server {
        &mut self.server
    }
}

impl_error!(ClientInterface, ClientInterfaceError);

#[derive(Debug, Error, Display)]
pub(crate) enum ClientInterfaceError {
    LoginFailed,
    NotLoggedIn,
    ServerError(#[from] ServerError)
}

impl IsClientInterface for ClientInterface {
    type Server = Server;
    type User = User;
    type Group = Group;
    type LoginCredentials = LoginCredentials;

    fn is_logged_in(&self) -> Result<bool, Self::Error> {
        Ok(self.login.is_some())
    }

    fn login_as(&mut self, cred: &Self::LoginCredentials) -> Result<(), Self::Error> {
        let user_id = self.server.login(cred.to_owned())?;
        self.login = Some(user_id);
        Ok(())
    }

    fn logout(&mut self) -> Result<(), Self::Error> {
        self.login = None;
        Ok(())
    }

    fn get_this_user_data(&self) -> Result<UserData, Self::Error> {
        if let Some(user_id) = self.login.as_ref() {
            Ok(self.server.find_user(user_id)?.get_data().map_err(ServerError::from)?.clone())
        } else {
            Err(ClientInterfaceError::NotLoggedIn)
        }
    }

    fn get_other_user_data(
        &self,
        user_id: &id_of!(Self::User),
    ) -> Result<UserData, Self::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub(crate) struct LoginCredentials {
    user_name: String,
    password: String
}

impl_error!(LoginCredentials, ClientInterfaceError);

impl IsLoginCred for LoginCredentials {
    type Password = String;
    type EncryptedPassword = String;

    fn new(&self, user_name: &str, password: &Self::Password) -> Result<Self, Self::Error> {
        Ok(Self {
            user_name: user_name.into(),
            password: password.into(),
        })
    }

    fn get_user_name(&self) -> Result<&str, Self::Error> {
        Ok(self.user_name.as_str())
    }

    fn get_password(&self) -> Result<&Self::EncryptedPassword, Self::Error> {
        Ok(&self.password)
    }
}