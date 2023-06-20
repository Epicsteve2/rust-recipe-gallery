FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as builder

# I don't think I need. Just bash into the postgres image
# RUN apt-get update \
#     && apt-get -y --no-install-recommends install postgresql \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

# ENV NODE_VERSION="${VERSION:-"lts/hydrogen"}"
# ENV NVM_VERSION="${NVMVERSION:-"0.39.2"}"
# ENV NVM_DIR=${NVMINSTALLPATH:-"/usr/local/share/nvm"}

RUN cargo install diesel_cli --version 2.1.0 && \
    cargo install trunk --version 0.16.0 && \
    cargo install cargo-leptos --version 0.1.11

# cargo install cargo-generate --version 0.18.3
# /usr/local/cargo/bin/cargo-generate

FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/trunk /usr/local/cargo/bin/

USER vscode
# --allow-downgarde is if rustfmt or clippy isn't available for that version of nightly
# possible to use multi-stage??? nah doubt it'll work
RUN rustup toolchain install nightly-2023-06-18 --allow-downgrade --component rustfmt --component clippy && \
    rustup target add --toolchain nightly-2023-06-18 wasm32-unknown-unknown && \
    rustup default nightly-2023-06-18

SHELL ["/bin/bash", "--login", "-c"]
RUN curl -so- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash && \
    source /home/vscode/.nvm/nvm.sh && \
    nvm install lts/hydrogen && \
    npm install --global tailwindcss@3.3.2 sass@1.63.4 && \
    rm -rf /home/vscode/.nvm/.cache/* && \
    npm cache clean --force

#  @playwright/test@1.35.1

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#    && apt-get -y install clang lld \
#    && apt-get autoremove -y && apt-get clean -y

RUN export DEBIAN_FRONTEND=noninteractive && apt-get install --no-install-recommends -y libnss3 \
    libnspr4 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libdbus-1-3 \
    libatspi2.0-0 \
    libx11-6 \
    libxcomposite1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxrandr2 \
    libgbm1 \
    libxcb1 \
    libxkbcommon0 \
    libpango-1.0-0 \
    libcairo2 \
    libasound2 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
# npx playwright install
