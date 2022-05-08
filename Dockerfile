###############################################################################
## Builder
###############################################################################
FROM rust:latest AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

ARG UID=${UID:-1000}
ARG GID=${GID:-1000}
ARG TARGET=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGET
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y \
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        libssl-dev \
        && \
    addgroup --gid ${GID} --system dockerus && \
    adduser --uid ${UID} --system --home /app --ingroup dockerus --disabled-password dockerus && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY ./ .

RUN cargo build  --target x86_64-unknown-linux-musl --release

###############################################################################
## Final image
###############################################################################
FROM alpine:3.15

RUN apk add --update --no-cache \
            tini~=0.19 \
            tzdata~=2022a && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*
# Copy the user
COPY --from=builder /etc/passwd /etc/group /etc/shadow /etc/

# Set the work dir
WORKDIR /app

# Use an unprivileged user.
USER dockerus

# Copy our build
COPY --chown=dockerus:dockerus --from=builder /app/target/x86_64-unknown-linux-musl/release/croni /app/

ENTRYPOINT ["tini", "--"]
CMD ["/app/croni"]
