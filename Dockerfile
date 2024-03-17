###############################################################################
## Builder
###############################################################################
FROM rust:1.76 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

ARG TARGET=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGET \
    OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu" \
    OPENSSL_INCLUDE_DIR="/usr/include/openssl"

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/cronirs /app/cronirs

###############################################################################
## Final image
###############################################################################
FROM alpine:3.19

ENV USER=app \
    UID=10001

RUN apk add --update --no-cache \
            tzdata~=2024 \
            curl~=8.5 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/cronirs /app/

# Create the user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}" && \
    chown -R app:app /app

WORKDIR /app
USER app

CMD ["/app/cronirs"]
