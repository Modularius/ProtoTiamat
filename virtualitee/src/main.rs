mod interface;
mod server;

use clap::Parser;
use std::io::stdin;

use crate::{interface::Input, server::{ClientInterface, Server}};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// If set, then OpenTelemetry data is sent to the URL specified, otherwise the standard tracing subscriber is used.
    #[clap(long)]
    otel_endpoint: Option<String>,

    /// All OpenTelemetry spans are emitted with this as the "service.namespace" property. Can be used to track different instances of the pipeline running in parallel.
    #[clap(long, default_value = "")]
    otel_namespace: String,
}

fn main() {
    let args = Cli::parse();
    let mut client_interace = ClientInterface::new(Server::default());
    let mut input_string = String::new();

    loop {
        stdin().read_line(&mut input_string).expect("");
        match Input::try_parse_from(input_string.split_ascii_whitespace()) {
            Ok(input) => {
                input.enact(&mut client_interace);
            }
            Err(e) => {
                println!(
                    "Error: {}.\n{e}",
                    input_string
                        .split_ascii_whitespace()
                        .map(ToOwned::to_owned)
                        .collect::<Vec<String>>()
                        .join(".")
                );
            }
        }
    }
}
