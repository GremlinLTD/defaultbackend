FROM ekidd/rust-musl-builder:latest AS builder
ADD . /home/rust/src
RUN cargo build --release


FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/defaultbackend /server
ENTRYPOINT [ "/server" ]