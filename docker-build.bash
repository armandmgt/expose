#!/usr/bin/env bash

set -euo pipefail

docker buildx build \
  --platform linux/arm64/v8,linux/amd64 \
  .

docker tag $(docker image ls --filter=dangling=true --filter=label=expose=expose) ghcr.io/armandmgt/expose:latest
docker tag $(docker image ls --filter=dangling=true --filter=label=expose=exposed) ghcr.io/armandmgt/exposed:latest
docker push ghcr.io/armandmgt/expose:latest
docker push ghcr.io/armandmgt/exposed:latest
