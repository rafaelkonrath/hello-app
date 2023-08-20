FROM rustlang/rust:nightly as builder
WORKDIR /hello-app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies openssl & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hello-app /usr/bin/hello-app
USER 1000
CMD ["hello-app"]