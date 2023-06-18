FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# I don't think I need. Just bash into the postgres image
# RUN apt-get update \
#     && apt-get -y --no-install-recommends install postgresql \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

USER vscode
RUN cargo install diesel_cli --version 2.1.0 && \
    cargo install trunk --version 0.16.0 && \
    cargo install cargo-leptos --version 0.1.11
# --allow-downgarde is if rustfmt or clippy isn't available for that version of nightly
RUN rustup toolchain install nightly-2023-06-18 --allow-downgrade --component rustfmt --component clippy && \
    rustup target add wasm32-unknown-unknown && \
    rustup default nightly-2023-06-18

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#    && apt-get -y install clang lld \
#    && apt-get autoremove -y && apt-get clean -y
