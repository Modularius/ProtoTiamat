#![allow(unused_crate_dependencies)]
use cfg_if::cfg_if;
use leptos::prelude::*;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use clap::Parser;
        use communitee::{App, ClientSideData, DefaultData, ServerSideData, shell, Server, PublicUrl};
        use libertee::RandomGeneration;
        use std::net::SocketAddr;
        use std::sync::{Arc, Mutex};
        use tracing::info;
        use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt};

        #[derive(Parser)]
        #[clap(author, version, about)]
        struct Cli {
            /// If set, then OpenTelemetry data is sent to the URL specified, otherwise the standard tracing subscriber is used.
            #[clap(long)]
            otel_endpoint: Option<String>,

            /// All OpenTelemetry spans are emitted with this as the "service.namespace" property. Can be used to track different instances of the pipeline running in parallel.
            #[clap(long, default_value = "")]
            otel_namespace: String,

            /// Endpoint on which OpenMetrics flavour metrics are available.
            #[clap(long, default_value = "127.0.0.1:9090")]
            observability_address: SocketAddr,

            #[clap(flatten)]
            default_data: DefaultData,

            /// Origin of the host from which the app is served (without the trailing slash).
            #[clap(long, default_value = "http://localhost:3000/")]
            public_url: PublicUrl,
        }

        #[actix_web::main]
        async fn main() -> miette::Result<()> {
            use actix_files::Files;
            use leptos_actix::{generate_route_list, LeptosRoutes};
            use miette::IntoDiagnostic;

            // set up logging
            console_error_panic_hook::set_once();

            let stdout_tracer = tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(false)
                .with_target(false);

            // This filter is applied to the stdout tracer
            let log_filter = EnvFilter::from_default_env();

            let subscriber =
                tracing_subscriber::Registry::default().with(stdout_tracer.with_filter(log_filter));

            //  This is only called once, so will never panic
            tracing::subscriber::set_global_default(subscriber)
                .expect("tracing::subscriber::set_global_default should only be called once");

            let args = Cli::parse();

            let server_side_data = ServerSideData {
                server: Arc::new(Mutex::new(Server::new_random(Default::default())))
                //entitee: Arc::new(Mutex::new(None)),
                //unitee: UniteeNode::default()
            };

            let client_side_data = ClientSideData {
                default_data: args.default_data,
                public_url: args.public_url
            };

            // Spawn the "purge expired sessions" task.

            let conf = get_configuration(None).unwrap();
            let addr = conf.leptos_options.site_addr;

            actix_web::HttpServer::new(move || {
                // Generate the list of routes in your Leptos App
                let routes = generate_route_list({
                    let client_side_data = client_side_data.clone();
                    move || {
                        provide_context(client_side_data.clone());
                        view!{ <App /> }
                    }
                });
                let leptos_options = &conf.leptos_options;
                let site_root = leptos_options.site_root.clone().to_string();

                info!("listening on http://{}", &addr);
                actix_web::App::new()
                    //.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .leptos_routes_with_context(routes, {
                        let server_side_data = server_side_data.clone();
                        let client_side_data = client_side_data.clone();
                        move ||{
                            provide_context(server_side_data.clone());
                            provide_context(client_side_data.clone());
                        }
                    }, {
                        let leptos_options = leptos_options.clone();
                        move ||shell(leptos_options.clone())
                    })
                    .app_data(actix_web::web::Data::new(leptos_options.to_owned()))
            })
            .bind(&addr)
            .into_diagnostic()?
            .run()
            .await
            .into_diagnostic()
        }
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {
    use communitee as _;
    use console_error_panic_hook as _;
    mount_to_body(|| {
        view! {
            "Please run using SSR"
        }
    });
}
