## make sure to expose only the ports you use in your deployment scenario!
let params = [
    "-d",
    "--name", "jaeger",
    "-p", "16686:16686",
    "-p", "4317:4317",
    "-p", "4318:4318",
    "-p", "5778:5778",
    "-p", "9411:9411",
    "-v", "/home/daniel/ProtoTiamat/scripts/all-in-one.yaml:/jaeger/config.yaml",
    "cr.jaegertracing.io/jaegertracing/jaeger:2.15.0",
    "--config", "/jaeger/config.yaml"
]
podman run ...$params