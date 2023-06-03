FROM rust:slim AS builder

RUN apt update
RUN update-ca-certificates

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM alpine
COPY --from=builder /usr/src/app/target/release/exposed ./
CMD [ "./exposed" ]
LABEL expose="exposed"

FROM alpine
COPY --from=builder /usr/src/app/target/release/expose ./
CMD [ "./expose" ]
LABEL expose="expose"