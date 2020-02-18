FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/backend
ENV CARGO_HOME=/usr/src/backend/.cargo
ENV ROCKET_ENV=prod
COPY . .
RUN cargo +nightly build --release

FROM debian:latest
EXPOSE 8001
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin
RUN apt-get update && apt-get install -y libssl-dev
ENTRYPOINT ["/usr/local/bin/backend"]