# --------------------------------------------------------------------------------------
# Build Container
# --------------------------------------------------------------------------------------

FROM --platform=$BUILDPLATFORM rust:1.87 AS builder
ARG TARGETPLATFORM

WORKDIR /usr/src

COPY Cargo.toml Cargo.lock ./
COPY src ./src/

RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release && \
    cp target/release/aoc-mcp-server-rs /usr/local/bin/


# --------------------------------------------------------------------------------------
# Distroless Release Container
# --------------------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /usr/local/bin/aoc-mcp-server-rs /opt/aoc-mcp-server-rs

EXPOSE 8123

ENTRYPOINT ["/opt/aoc-mcp-server-rs"]
