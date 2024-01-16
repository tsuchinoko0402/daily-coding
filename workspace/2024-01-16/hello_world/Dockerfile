FROM rust:1.75-bookworm as builder
WORKDIR /opt/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /opt/app/target/release/hello_world . 
CMD [ "./hello_world" ] 