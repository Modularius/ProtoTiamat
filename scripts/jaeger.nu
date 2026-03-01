## make sure to expose only the ports you use in your deployment scenario!
#let config = "/home/daniel/ProtoTiamat/scripts/all-in-one.yaml:/jaeger/config.yaml";
let config = "/home/drdan/Documents/Rust/ProtoTiamat/scripts/all-in-one.yaml:/jaeger/config.yaml";
let params = [
    "-d",
    "--name", "jaeger",
    "-e", "COLLECTOR_OTLP_ENABLED=true",
    "-e", "COLLECTOR_OTLP_GRPC_HOST_PORT=0.0.0.0:14317",
    "-e", "COLLECTOR_OTLP_HTTP_HOST_PORT=0.0.0.0:14318",
    "-p", "16686:16686",
    "-p", "4317:4317",
    "-p", "4318:4318",
    "-p", "5778:5778",
    "-p", "9411:9411",
    "-v", $config,
    "cr.jaegertracing.io/jaegertracing/jaeger:2.15.0",
    "--config", "/jaeger/config.yaml"
]
podman run ...$params