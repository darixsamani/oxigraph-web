FROM rust:1.90 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --target-dir target



FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/oxigraph_web /app/oxigraph_web


EXPOSE 8080

CMD ["/app/oxigraph_web"]