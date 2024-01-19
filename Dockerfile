FROM rust:1.67
WORKDIR /opt/hookd
COPY . .
RUN cargo build --release
RUN chmod +x ./target/release/hookd
RUN rm -rf target/release/deps target/release/build
CMD ["./target/release/hookd"]