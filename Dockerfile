# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.96.0

################################################################################
# cargo-chef splits the build so compiled dependencies get their own layer, which only
# changes with Cargo.toml/Cargo.lock and is reused from the registry/GHA cache in CI.
#
# qr-encode comes from a private GitHub repository over SSH. Build with an SSH agent
# that holds a key with read access: `docker build --ssh default .` (CI passes a
# deploy key the same way).

FROM rust:${RUST_VERSION}-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends git openssh-client \
    && rm -rf /var/lib/apt/lists/*
# Pin GitHub's published host key (api.github.com/meta) instead of trusting on first use.
RUN mkdir -p -m 0700 /root/.ssh \
    && echo "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl" \
        > /root/.ssh/known_hosts
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install cargo-chef --locked --version 0.1.71

################################################################################
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=ssh \
    cargo chef cook --release --locked --package zpl-http-api --recipe-path recipe.json

COPY . .
RUN --mount=type=ssh \
    cargo build --release --locked --package zpl-http-api --bin zpl-server \
    && cp target/release/zpl-server /zpl-server

################################################################################
# The /playground editor: zpl-wasm compiled to WebAssembly next to its static page,
# with a .gz of every file so the server never compresses at request time.
FROM chef AS wasm
# wasm-bindgen-cli must match the wasm-bindgen version in Cargo.lock.
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli --locked --version 0.2.106
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=ssh \
    cargo chef cook --release --locked --target wasm32-unknown-unknown --package zpl-wasm --recipe-path recipe.json

COPY . .
RUN --mount=type=ssh \
    cargo build --release --locked --target wasm32-unknown-unknown --package zpl-wasm \
    && wasm-bindgen --target web --no-typescript --out-dir zpl-wasm/www/pkg \
        target/wasm32-unknown-unknown/release/zpl_wasm.wasm \
    && mkdir /playground \
    && cp -R zpl-wasm/www/. /playground/ \
    && find /playground -type f \( -name '*.html' -o -name '*.js' -o -name '*.wasm' \) -exec gzip -9k {} +

################################################################################
# glibc runtime (not musl: its allocator noticeably slows rendering), no shell, non-root.
FROM gcr.io/distroless/cc-debian12:nonroot AS final
COPY --from=build /zpl-server /zpl-server
COPY --from=wasm /playground /playground
ENV ZPL_API_ADDR=0.0.0.0:3000
ENV ZPL_PLAYGROUND_DIR=/playground
EXPOSE 3000
USER nonroot
ENTRYPOINT ["/zpl-server"]
