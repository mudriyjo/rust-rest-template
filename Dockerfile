FROM rust:1.84 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/server /server
CMD ["/server"]