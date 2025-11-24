FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app

RUN useradd -ms /bin/false appuser
USER appuser

COPY --from=builder /app/target/release/barcode-scanner-http-client /app/barcode-scanner-http-client
CMD ["/app/barcode-scanner-http-client"]
