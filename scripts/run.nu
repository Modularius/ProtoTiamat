let params = [
    "--initial-user-name", "Fred",
    "--initial-user-username", "FredieFuckah",
    "--initial-user-password", "Bozos"
]
RUST_LOG=info cargo leptos watch -p communitee -- ...$params