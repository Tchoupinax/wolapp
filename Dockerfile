FROM rust:1.74 as builder

WORKDIR /wolapp

COPY . .

RUN cargo build --release

###
###
###

FROM scratch

WORKDIR /app

COPY --from=builder /wolapp/target/release/wolapp /app/wolapp
COPY views /app/views

EXPOSE 5644

CMD ["/app/wolapp"]
