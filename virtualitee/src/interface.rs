use clap::{Parser, Subcommand};

use crate::server::{ClientInterface, Server};
use libertee::traits::{IsAdminInterface, IsServer, IsUser, IsUserData};

pub(crate) trait Enactable: Subcommand {
    fn enact(self, client_interace: &mut ClientInterface);
}

#[derive(Parser)]
pub(crate) struct ParserWrapper<C: Subcommand> {
    /// If set, then OpenTelemetry data is sent to the URL specified, otherwise the standard tracing subscriber is used.
    #[command(subcommand)]
    command: C,
}

impl<C: Enactable> ParserWrapper<C> {
    pub(crate) fn enact(self, client_interace: &mut ClientInterface) {
        self.command.enact(client_interace);
    }
}

pub(crate) type Input = ParserWrapper<Command>;

#[derive(Subcommand)]
pub(crate) enum Command {
    PrintUsers,
    PrintGroups,
    LoginAsUser { user: String },
    User(ParserWrapper<UserCommand>),
}

impl Enactable for Command {
    fn enact(self, client_interace: &mut ClientInterface) {
        match self {
            Self::PrintUsers => {
                for user in client_interace.get_server_mut().iter_user().unwrap() {
                    println!("{}", user.get_data().unwrap().get_name().unwrap())
                }
            }
            Self::PrintGroups => {
                for user in client_interace.get_server_mut().iter_group().unwrap() {
                    println!("{}", user.get_data().unwrap().get_name().unwrap())
                }
            }
            Self::LoginAsUser { user } => {

            }
            Self::User(user) => user.command.enact(client_interace),
        }
    }
}

#[derive(Subcommand)]
pub(crate) enum UserCommand {
    ListFriends,
    ListBlocked,
    ListGroups,
    DisplayNewFeed,
    ExtendFeed,
}

impl Enactable for UserCommand {
    fn enact(self, client_interface: &mut ClientInterface) {
        match self {
            Self::ListFriends => {}
            Self::ListBlocked => {}
            Self::ListGroups => {}
            Self::DisplayNewFeed => {}
            Self::ExtendFeed => {}
        }
    }
}
