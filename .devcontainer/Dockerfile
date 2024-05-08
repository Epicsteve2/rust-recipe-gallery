# a lot of stages b/c of compile times...
# TODO: Update to bookwork soon?
FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as diesel
RUN cargo install diesel_cli --version 2.1.1
# FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as leptos
# RUN cargo install cargo-leptos --version 0.1.11
# FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as trunk
# RUN cargo install trunk --version 0.17.3

FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as cargo-leptos
ADD https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz /
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz && cp cargo-binstall /usr/local/cargo/bin && \
  cargo binstall --no-track --no-confirm cargo-leptos --version 0.2.17
# cargo binstall --no-track --no-confirm trunk --version 0.17.3 # missing glibc
# cargo binstall --no-track --no-confirm diesel_cli --version 2.1.0 && \ # doesn't work... doesn't have binaries, or isn't compiled correctly
# RUN cargo install wasm-pack --version 0.12.1 # nah
# RUN cargo install cargo-generate --version 0.18.3 # prolly not needed at all

# I don't think I need. Just bash into the postgres image
# RUN apt-get update \
#     && apt-get --yes --no-install-recommends install postgresql \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

# ENV NODE_VERSION="${VERSION:-"lts/hydrogen"}"
# ENV NVM_VERSION="${NVMVERSION:-"0.39.2"}"
# ENV NVM_DIR=${NVMINSTALLPATH:-"/usr/local/share/nvm"}

# cargo install cargo-generate --version 0.18.3
# /usr/local/cargo/bin/cargo-generate

FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as node

# TODO do this? https://stackoverflow.com/a/66390227 https://github.com/nodejs/help/wiki/Installation
# nvm nah
# ARG NODE_VERSION=18.17.0
# ARG NODE_PACKAGE=node-v$NODE_VERSION-linux-x64
# ARG NODE_HOME=/usr/lib/$NODE_PACKAGE
# ADD https://nodejs.org/dist/v$NODE_VERSION/$NODE_PACKAGE.tar.gz /
# RUN tar -xzf $NODE_PACKAGE --directory=/usr/local/lib/
# # --extract --gzip

# ENV NODE_PATH $NODE_HOME/lib/node_modules
# ENV PATH $NODE_HOME/bin:$PATH
USER vscode
SHELL ["/bin/bash", "--login", "-c"]
RUN curl --silent --output - https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash && \
  source /home/vscode/.nvm/nvm.sh && \
  nvm install v20.13.0 && \
  rm -rf /home/vscode/.nvm/.cache/*

RUN source /home/vscode/.nvm/nvm.sh && \
  npm install --global @playwright/test@1.44.0 && \
  playwright install && \
  npm uninstall --global @playwright/test && \
  npm cache clean --force

RUN source /home/vscode/.nvm/nvm.sh && \
  npm install --global tailwindcss@3.3.3 sass@1.77.0 && \
  npm cache clean --force

FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye as rust-nightly
RUN rustup target add --toolchain 1.77.2 wasm32-unknown-unknown

FROM mcr.microsoft.com/devcontainers/rust:1.0.11-bullseye
# playwright dependenecies
# doesn't work for debian 12 aka bookworm
# to get dependencies, run `playwright install-deps --dry-run`
RUN export DEBIAN_FRONTEND=noninteractive && \
  apt-get update && \
  apt-get install --no-install-recommends --yes \
  libasound2 \
  libatk-bridge2.0-0 \
  libatk1.0-0 \
  libatspi2.0-0 \
  libcairo2 \
  libcups2 \
  libdbus-1-3 \
  libdrm2 \
  libgbm1 \
  libglib2.0-0 \
  libnspr4 \
  libnss3 \
  libpango-1.0-0 \
  libwayland-client0 \
  libx11-6 \
  libxcb1 \
  libxcomposite1 \
  libxdamage1 \
  libxext6 \
  libxfixes3 \
  libxkbcommon0 \
  libxrandr2 \
  xvfb \
  fonts-noto-color-emoji \
  fonts-unifont \
  libfontconfig1 \
  libfreetype6 \
  xfonts-cyrillic \
  xfonts-scalable \
  fonts-liberation \
  fonts-ipafont-gothic \
  fonts-wqy-zenhei \
  fonts-tlwg-loma-otf \
  fonts-freefont-ttf \
  libcairo-gobject2 \
  libdbus-glib-1-2 \
  libgdk-pixbuf-2.0-0 \
  libgtk-3-0 \
  libharfbuzz0b \
  libpangocairo-1.0-0 \
  libx11-xcb1 \
  libxcb-shm0 \
  libxcursor1 \
  libxi6 \
  libxrender1 \
  libxtst6 \
  gstreamer1.0-libav \
  gstreamer1.0-plugins-bad \
  gstreamer1.0-plugins-base \
  gstreamer1.0-plugins-good \
  libegl1 \
  libenchant-2-2 \
  libepoxy0 \
  libevdev2 \
  libgles2 \
  libglx0 \
  libgstreamer-gl1.0-0 \
  libgstreamer-plugins-base1.0-0 \
  libgstreamer1.0-0 \
  libgudev-1.0-0 \
  libharfbuzz-icu0 \
  libhyphen0 \
  libicu67 \
  libjpeg62-turbo \
  liblcms2-2 \
  libmanette-0.2-0 \
  libnotify4 \
  libopengl0 \
  libopenjp2-7 \
  libopus0 \
  libpng16-16 \
  libproxy1v5 \
  libsecret-1-0 \
  libsoup2.4-1 \
  libwayland-egl1 \
  libwayland-server0 \
  libwebp6 \
  libwebpdemux2 \
  libwoff1 \
  libxml2 \
  libxslt1.1 \
  libatomic1 \
  libevent-2.1-7 && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

USER vscode
# RUN rustup toolchain install 1.77.2 --allow-downgrade --component rustfmt --component clippy --component rust-src && \
#   rustup target add --toolchain 1.77.2 wasm32-unknown-unknown && \
#   rustup default 1.77.2

COPY --from=node --chown=vscode:vscode /home/vscode/.cache /home/vscode/.cache/
## this increased file size by like 600 mb... i really don't know why. my guess is that i'm adding more than 600mb of files. namely, 1.2gb because of previous files?
COPY --from=rust-nightly --chown=vscode:vscode /usr/local/rustup/toolchains/1.77.2-x86_64-unknown-linux-gnu /usr/local/rustup/toolchains/1.77.2-x86_64-unknown-linux-gnu
COPY --from=rust-nightly --chown=vscode:vscode /usr/local/rustup/update-hashes/1.77.2-x86_64-unknown-linux-gnu /usr/local/rustup/update-hashes/
COPY --from=rust-nightly --chown=vscode:vscode /usr/local/rustup/settings.toml /usr/local/rustup/
COPY --from=node --chown=vscode:vscode /home/vscode/.nvm /home/vscode/.nvm/
COPY --from=diesel /usr/local/cargo/bin/diesel /usr/local/cargo/bin/
COPY --from=cargo-leptos /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/
COPY --from=node --chown=vscode:vscode /home/vscode/.bashrc /home/vscode/

# nah i got the command above
# echo 'debconf debconf/frontend select Noninteractive' | sudo debconf-set-selections && \
# cd && \
# playwright install-deps && \
# playwright install && \
# npm uninstall playwright && \
# npm cache clean --force && \
# sudo apt-get clean && \
# sudo rm -rf /var/lib/apt/lists/*

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#    && apt-get -y install clang lld \
#    && apt-get autoremove -y && apt-get clean -y
