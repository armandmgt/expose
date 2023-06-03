FROM rust:slim AS builder

RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN curl -fsSL https://deb.nodesource.com/setup_current.x | sudo -E bash -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        nodejs yarn
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