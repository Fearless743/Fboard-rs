# Build from the repository root:
#   docker build -f rust/Dockerfile -t fboard-rs .
#
# The admin SPA is optional at build time. To embed it, build the Fboard admin
# first so public/assets/admin exists, then build this image.

FROM docker.io/library/rust:1-bookworm AS builder
WORKDIR /app

COPY rust/Cargo.toml rust/Cargo.lock ./rust/
COPY rust/src ./rust/src
COPY rust/migrations ./rust/migrations
COPY resources ./resources

RUN cargo build --release --manifest-path rust/Cargo.toml

FROM docker.io/library/debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --home-dir /app fboard

WORKDIR /app
COPY --from=builder /app/rust/target/release/fboard /usr/local/bin/fboard
COPY resources ./resources
COPY public ./public

RUN mkdir -p /app/plugins /app/storage /app/public/assets \
    && chown -R fboard:fboard /app

ENV FBOARD_LISTEN=0.0.0.0:7001 \
    FBOARD_ADMIN_DIST=/app/public/assets/admin \
    FBOARD_PLUGIN_DIRS=/app/plugins \
    FBOARD_UPLOAD_DIR=/app/storage

USER fboard
EXPOSE 7001

ENTRYPOINT ["fboard"]
CMD ["serve"]
