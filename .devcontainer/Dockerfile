FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as builder

# I don't think I need. Just bash into the postgres image
# RUN apt-get update \
#     && apt-get -y --no-install-recommends install postgresql \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --version 2.1.0 && \
    cargo install trunk --version 0.16.0 && \
    cargo install cargo-leptos --version 0.1.11

# Also might need npm... for Tailwind

FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/trunk /usr/local/cargo/bin/

USER vscode
# --allow-downgarde is if rustfmt or clippy isn't available for that version of nightly
# possible to use multi-stage??? nah doubt it'll work
RUN rustup toolchain install nightly-2023-06-18 --allow-downgrade --component rustfmt --component clippy && \
    rustup target add --toolchain nightly-2023-06-18 wasm32-unknown-unknown && \
    rustup default nightly-2023-06-18

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#    && apt-get -y install clang lld \
#    && apt-get autoremove -y && apt-get clean -y
