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
# glibc runtime (not musl: its allocator noticeably slows rendering), no shell, non-root.
FROM gcr.io/distroless/cc-debian12:nonroot AS final
COPY --from=build /zpl-server /zpl-server
ENV ZPL_API_ADDR=0.0.0.0:3000
EXPOSE 3000
USER nonroot
ENTRYPOINT ["/zpl-server"]
