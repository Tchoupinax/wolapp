FROM rust:1.72.1 as builder

WORKDIR /wolapp

COPY . .

RUN cargo build --release

###

FROM debian

WORKDIR /app

COPY --from=builder /wolapp/target/release/wolapp /app/wolapp

COPY views /app/views

EXPOSE 5644

CMD ["/app/wolapp"]
