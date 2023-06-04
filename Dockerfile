FROM rust:1-bullseye AS builder

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
      apt-get -y install --no-install-recommends \
      musl-tools clang llvm

RUN update-ca-certificates

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

ARG ARCH
RUN rustup target add ${ARCH}-unknown-linux-musl

WORKDIR /usr/src/
RUN USER=root cargo new app

WORKDIR /usr/src/app/
COPY Cargo.toml Cargo.lock /usr/src/app/

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# This is a dummy build to get the dependencies cached.
RUN cargo build --target ${ARCH}-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY . /usr/src/app/

# This is the actual application build.
RUN cargo build --target ${ARCH}-unknown-linux-musl --release


FROM alpine AS runtime

WORKDIR /app/

ARG ARCH
COPY --from=builder /usr/src/app/target/${ARCH}-unknown-linux-musl/release/expose /usr/bin/expose

ENTRYPOINT [ "/usr/bin/expose" ]