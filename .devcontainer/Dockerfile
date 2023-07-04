# a lot of stages b/c of compile times...
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as diesel
RUN cargo install diesel_cli --version 2.1.0
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as leptos
RUN cargo install cargo-leptos --version 0.1.11
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye as trunk
RUN cargo install trunk --version 0.17.1

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

FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
COPY --from=diesel /usr/local/cargo/bin/diesel /usr/local/cargo/bin/
COPY --from=leptos /usr/local/cargo/bin/cargo-leptos /usr/local/cargo/bin/
COPY --from=trunk /usr/local/cargo/bin/trunk /usr/local/cargo/bin/

# playwright dependenecies
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install --no-install-recommends --yes \
    libasound2 \
    libatk-bridge2.0-0 \
    libatspi2.0-0 \
    libatk1.0-0 \
    libcairo-gobject2 \
    libcairo2 \
    libcups2 \
    libdbus-1-3 \
    libdbus-glib-1-2 \
    libenchant-2-2 \
    libfontconfig1 \
    fonts-freefont-ttf \
    fonts-ipafont-gothic \
    fonts-liberation \
    fonts-noto-color-emoji \
    fonts-tlwg-loma-otf \
    fonts-wqy-zenhei \
    libfreetype6 \
    libatomic1 \
    libgdk-pixbuf-2.0-0 \
    libglib2.0-0 \
    gstreamer1.0-libav \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-base \
    libgstreamer-gl1.0-0 \
    libgstreamer-plugins-base1.0-0 \
    gstreamer1.0-plugins-good \
    libgstreamer1.0-0 \
    libgtk-3-0 \
    libharfbuzz-icu0 \
    libharfbuzz0b \
    libhyphen0 \
    libicu67 \
    liblcms2-2 \
    libdrm2 \
    libepoxy0 \
    libevdev2 \
    libevent-2.1-7 \
    libegl1 \
    libgles2 \
    libglx0 \
    libopengl0 \
    libgudev-1.0-0 \
    libjpeg62-turbo \
    libmanette-0.2-0 \
    libnotify4 \
    libpng16-16 \
    libproxy1v5 \
    libsecret-1-0 \
    libsoup2.4-1 \
    libx11-6 \
    libx11-xcb1 \
    libxcb-shm0 \
    libxcb1 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxkbcommon0 \
    libxml2 \
    libxrandr2 \
    libxrender1 \
    libxslt1.1 \
    libxtst6 \
    libgbm1 \
    libnspr4 \
    libnss3 \
    libopenjp2-7 \
    libopus0 \
    libpango-1.0-0 \
    libpangocairo-1.0-0 \
    fonts-unifont \
    libwayland-client0 \
    libwayland-egl1 \
    libwayland-server0 \
    libwoff1 \
    xfonts-cyrillic \
    xfonts-scalable \
    xvfb \
    libwebp6 \
    libwebpdemux2 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER vscode
# --allow-downgarde is if rustfmt or clippy isn't available for that version of nightly
# possible to use multi-stage??? nah doubt it'll work
RUN rustup toolchain install nightly-2023-07-03 --allow-downgrade --component rustfmt --component clippy && \
    rustup target add --toolchain nightly-2023-07-03 wasm32-unknown-unknown && \
    rustup default nightly-2023-07-03

SHELL ["/bin/bash", "--login", "-c"]
RUN curl --silent --output - https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash && \
    source /home/vscode/.nvm/nvm.sh && \
    nvm install lts/hydrogen && \
    npm install --global tailwindcss@3.3.2 sass@1.63.4 @playwright/test@1.35.1 && \
    rm -rf /home/vscode/.nvm/.cache/* && \
    playwright install && \
    npm uninstall --global @playwright/test && \
    npm cache clean --force

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
