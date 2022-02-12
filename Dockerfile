FROM rust:1.58.1
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/bodhi-bot-rs"]
