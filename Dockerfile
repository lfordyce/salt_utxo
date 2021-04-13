ARG BUILDER_IMAGE=rust:latest
#############################
## STEP 1 build executable binary
#############################
FROM ${BUILDER_IMAGE} as builder

WORKDIR /usr/src

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y libssl-dev && \
    rustup toolchain add stable

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
RUN cargo build --release

############################
# STEP 2 build a small image
############################
FROM debian:10-slim

# Import from builder.
COPY --from=builder /usr/src/target/release/salt_utxo .

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y libssl-dev

USER 1000
CMD ["./salt_utxo"]
