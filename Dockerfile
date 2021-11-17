# build stage
FROM debian:buster-slim as runner

RUN apt update; apt install -y libssl1.1 libpq5

FROM rust:1.48.0 as builder

RUN rustup default nightly

WORKDIR /usr/src/resto_app
COPY . .

RUN cargo install --path .

FROM runner
COPY --from=builder /usr/local/cargo/bin/resto_app .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./resto_app"]
