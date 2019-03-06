FROM rust:1.33

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8080

VOLUME ["/usr/local/cargo"]