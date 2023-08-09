FROM rust:1.71-slim-bullseye as builder
RUN cargo new rust-recipe-gallery
WORKDIR /rust-recipe-gallery
RUN cargo new rust-recipe-gallery-backend && cargo new rust-recipe-gallery-frontend

# I think this can be optimized, running cargo build --release still looks at toolchain and target stuff...
RUN rustup toolchain install --profile=minimal nightly-2023-08-08 && rustup target add --toolchain nightly-2023-08-08 wasm32-unknown-unknown
COPY ./Cargo.toml ./Cargo.lock ./rust-toolchain.toml ./
COPY ./rust-recipe-gallery-backend/Cargo.toml ./rust-recipe-gallery-backend/
COPY ./rust-recipe-gallery-frontend/Cargo.toml ./rust-recipe-gallery-frontend/
RUN cargo build --release

# Build web app with own code
RUN rm src/*.rs rust-recipe-gallery-backend/src/*.rs ./rust-recipe-gallery-frontend/src/*.rs ./target/release/deps/rust_recipe_gallery*
ADD . ./
RUN cargo build --release

#
FROM debian:bullseye-slim
EXPOSE 7979
COPY --from=builder /rust-recipe-gallery/target/release/rust-recipe-gallery-backend /rust-recipe-gallery/
CMD ["/rust-recipe-gallery/rust-recipe-gallery-backend"]
