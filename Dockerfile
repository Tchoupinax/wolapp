FROM rust:1.68.2

RUN USER=root cargo new --bin wolapp
WORKDIR /wolapp

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/wolapp*
RUN cargo install --path .

CMD ["wolapp"]
