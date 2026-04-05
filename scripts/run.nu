let params = [
    "--otel-endpoint", "http://localhost:4317",
    "--otel-namespace", "communitee",
    "--initial-user-name", "Fred",
    "--initial-user-username", "FredieFuckah",
    "--initial-user-password", ""
]
RUST_BACKTRACE=1 RUST_LOG=debug OTEL_LEVEL=info,actix_web=info cargo leptos watch -p tiamat -- ...$params