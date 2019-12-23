FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/backend
ENV CARGO_HOME=/usr/src/backend/.cargo
ENV ROCKET_ENV=stage
COPY . .
RUN cargo +nightly build --release

FROM debian:latest
EXPOSE 8000
ENV RUST_BACKTRACE=1
ENV DATABASE_URL=mysql://root:pass@localhost:3306/ferienwvk_db1
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/backend"]