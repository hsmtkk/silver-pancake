FROM rust:latest AS builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11 AS runtime
COPY --from=builder /opt/target/release/silver-pancake /usr/local/bin/silver-pancake
ENTRYPOINT ["/usr/local/bin/silver-pancake"]
