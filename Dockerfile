FROM rust:1-bullseye AS builder

RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN curl -fsSL https://deb.nodesource.com/setup_current.x | bash -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
      apt-get -y install --no-install-recommends \
      nodejs yarn

RUN update-ca-certificates

WORKDIR /usr/src/app

COPY . .

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN --mount=type=cache,target=/root/.cargo \
      ["cargo", "build", "--release"]

ARG ${BIN_NAME}
FROM alpine
COPY --from=builder /usr/src/app/target/release/${BIN_NAME} ./
CMD [ "./${BIN_NAME}" ]

