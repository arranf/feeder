FROM rust:1.33

RUN cargo install

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]