FROM rust:1-bullseye AS builder

RUN update-ca-certificates

WORKDIR /usr/src/app

COPY . .

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release


FROM alpine

COPY --from=builder /usr/src/app/target/release/expose /usr/bin/expose

ENTRYPOINT [ "/usr/bin/expose" ]
