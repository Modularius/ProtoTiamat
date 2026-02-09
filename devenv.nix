{pkgs, ...}: {
  packages = with pkgs; [
    # Rust toolchain
    rustup
    clang
    cmake
    pkg-config
    cargo-leptos
    cargo-binutils

    # Code formatting tools
    alejandra
    treefmt
    mdl

    # Dependency auditing
    cargo-deny

    # Additional toolchain components
    binaryen
    dart-sass

    podman
    podman-compose
    buildah
    openssl
    su
    nushell
    tailwindcss_4
  ];

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
