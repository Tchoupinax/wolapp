FROM rust:1.68.2 as builder

RUN USER=root cargo new --bin wolapp
WORKDIR /wolapp

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

###

FROM scratch

WORKDIR /app

COPY --from=builder /wolapp/target/release/wolapp /app/wolapp

EXPOSE 3000

CMD ["/app/wolapp"]
