# Credits:
# https://www.21analytics.ch/blog/docker-from-scratch-for-rust-applications/

FROM rust:1.74-alpine as builder
RUN apk add --no-cache musl-dev cmake pkgconf make g++

# Set `SYSROOT` to a dummy path (default is /usr) because pkg-config-rs *always*
# links those located in that path dynamically but we want static linking, c.f.
# https://github.com/rust-lang/pkg-config-rs/blob/54325785816695df031cef3b26b6a9a203bbc01b/src/lib.rs#L613
ENV SYSROOT=/dummy

WORKDIR /wd
COPY . /wd
RUN cargo build --bins --release

FROM scratch

COPY --from=builder /wd/target/release/send2kafka /send2kafka
ENTRYPOINT ["/send2kafka"]