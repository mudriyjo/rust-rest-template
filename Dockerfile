FROM rust:1.79

WORKDIR /usr/src/server
COPY . .
RUN cargo build --release
CMD ["/usr/src/server/target/release/server"]