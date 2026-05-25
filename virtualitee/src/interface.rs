use clap::{Parser, Subcommand};

pub(crate) trait Enactable: Subcommand {
    fn enact(self);
}

#[derive(Parser)]
pub(crate) struct ParserWrapper<C: Subcommand> {
    /// If set, then OpenTelemetry data is sent to the URL specified, otherwise the standard tracing subscriber is used.
    #[command(subcommand)]
    command: C,
}

impl<C: Enactable> ParserWrapper<C> {
    pub(crate) fn enact(self) {
        self.command.enact();
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
    fn enact(self) {
        match self {
            Self::PrintUsers => {}
            Self::PrintGroups => {}
            Self::LoginAsUser { user } => {}
            Self::User(user) => user.command.enact(),
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
    fn enact(self) {
        match self {
            Self::ListFriends => {}
            Self::ListBlocked => {}
            Self::ListGroups => {}
            Self::DisplayNewFeed => {}
            Self::ExtendFeed => {}
        }
    }
}
