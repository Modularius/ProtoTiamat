use clap::Parser;

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
    println!("Hello, world!");
}
