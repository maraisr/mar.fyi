FROM rust:latest as BUILDER

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update
RUN apt-get install -y \
    musl-dev \
	musl-tools

WORKDIR /app

ADD ["*.rs", "Cargo.*", "./"]

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch as RUNTIME

EXPOSE 8080

COPY --from=BUILDER /app/target/x86_64-unknown-linux-musl/release/app .

CMD ["./app"]
