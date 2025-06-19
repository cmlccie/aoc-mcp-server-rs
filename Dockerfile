# --------------------------------------------------------------------------------------
# Build Container
# --------------------------------------------------------------------------------------

FROM --platform=$BUILDPLATFORM rust:1.87 AS builder
ARG TARGETPLATFORM

WORKDIR /usr/src

COPY .cargo/config.toml .cargo/config.toml
COPY Cargo.toml Cargo.lock ./
COPY src ./src/

RUN case "$TARGETPLATFORM" in \
      "linux/amd64"*) \
        TARGET="x86_64-unknown-linux-gnu" ;; \
      "linux/arm64"*) \
        TARGET="aarch64-unknown-linux-gnu" ;; \
      *) \
        echo "ERROR: Unsupported platform: $TARGETPLATFORM" && \
        echo "Supported platforms are: linux/amd64, linux/arm64" && \
        exit 1 ;; \
    esac && \
    cargo build --release --target $TARGET && \
    cp target/$TARGET/release/aoc-mcp-server-rs /usr/local/bin/


# --------------------------------------------------------------------------------------
# Distroless Release Container
# --------------------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /usr/local/bin/aoc-mcp-server-rs /opt/aoc-mcp-server-rs

EXPOSE 8123

ENTRYPOINT ["/opt/aoc-mcp-server-rs"]
