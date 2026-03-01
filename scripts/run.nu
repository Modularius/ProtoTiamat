let params = [
    "--otel-endpoint", "http://localhost:4317",
    "--otel-namespace", "communitee",
    "--initial-user-name", "Fred",
    "--initial-user-username", "FredieFuckah",
    "--initial-user-password", "Bozos"
]
RUST_LOG=debug cargo leptos watch -p communitee -- ...$params