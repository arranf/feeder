# Copied from: https://github.com/emk/rust-musl-builder/blob/master/examples/using-diesel/Dockerfile

# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder AS builder

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `feeder`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/feeder \
    /usr/local/bin/

EXPOSE 8000
CMD /usr/local/bin/feeder