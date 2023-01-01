###############################################################################
## Builder
###############################################################################
FROM rust:latest AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM" > /log

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

COPY ./platform.sh /platform.sh
RUN /platform.sh

ENV RUST_MUSL_CROSS_TARGET=$TARGETPLATFORM

RUN rustup target add $(cat /.target) && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        libssl-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target $(cat /.target) && \
    cp /app/target/$(cat /.target)/release/cronirs /app/cronirs

###############################################################################
## Final image
###############################################################################
FROM alpine:3.17

RUN apk add --update --no-cache \
            su-exec~=0.2 \
            curl~=7.87 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*
# Copy the user

# Set the work dir
WORKDIR /app

COPY entrypoint.sh /app/
# Copy our build
COPY --from=builder /app/cronirs /app/

ENTRYPOINT ["/bin/sh", "/app/entrypoint.sh"]
CMD ["/app/cronirs"]
