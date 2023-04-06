FROM rust:1.68.2 as builder

WORKDIR /wolapp

COPY . .

RUN cargo build --release

###

FROM debian

WORKDIR /app

COPY --from=builder /wolapp/target/release/wolapp /app/wolapp

EXPOSE 5644

CMD ["/app/wolapp"]
