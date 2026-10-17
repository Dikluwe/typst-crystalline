FROM --platform=$BUILDPLATFORM tonistiigi/xx@sha256:40fe21f44bb8cddf1eb1226db190d0a984118f62ee02a91b77d99448d28264a6 AS xx
FROM --platform=$BUILDPLATFORM rust:1.92.0-alpine3.22@sha256:0fe89ba9d67dc20a4ff50206d25c88606919cb611b41dd0285dfe3cbb02d69de AS build

COPY --from=xx / /

RUN apk add --no-cache clang lld
COPY . /app
WORKDIR /app
RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    cargo fetch --locked

ARG TARGETPLATFORM
ARG REVISION
ENV TYPST_COMMIT_SHA=$REVISION

RUN xx-apk add --no-cache musl-dev
RUN --mount=type=cache,target=/root/.cargo/git/db \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/registry/index \
    xx-cargo build --locked -p typst-wiring --bin typst --release && \
    cp target/$(xx-cargo --print-target-triple)/release/typst target/release/typst && \
    xx-verify target/release/typst

FROM alpine:3.22@sha256:5291449c3df73caf6ed85e649dec1b9e818b39a5d8c871e97afc13e9cd5e8fa8
ARG CREATED
ARG REVISION

# Create a non-root user that can be activated with `--user typst`
RUN addgroup -g 1000 typst && \
    adduser -D -u 1000 -G typst typst

LABEL org.opencontainers.image.authors="The Typst Project Developers <hello@typst.app>"
LABEL org.opencontainers.image.created=${CREATED}
LABEL org.opencontainers.image.description="A markup-based typesetting system"
LABEL org.opencontainers.image.documentation="https://typst.app/docs"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.revision=${REVISION}
LABEL org.opencontainers.image.source="https://github.com/Dikluwe/typst-crystalline"
LABEL org.opencontainers.image.title="Typst Crystalline (Tekt)"
LABEL org.opencontainers.image.url="https://github.com/Dikluwe/typst-crystalline"
LABEL org.opencontainers.image.vendor="Typst"

COPY --from=build  /app/target/release/typst /bin
ENTRYPOINT [ "/bin/typst" ]
