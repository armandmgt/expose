#!/usr/bin/env bash

set -euo pipefail

docker buildx build \
  --push \
  --platform linux/arm64/v8,linux/amd64 \
  --build-arg BIN_NAME=expose \
  -t ghcr.io/armandmgt/expose:latest .

docker buildx build \
  --push \
  --platform linux/arm64/v8,linux/amd64 \
  --build-arg BIN_NAME=expose \
  -t ghcr.io/armandmgt/expose:latest .

