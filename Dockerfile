FROM rust:1.56.1 as builder


RUN USER=root cargo new --bin id-generator
WORKDIR /id-generator

#COPY ./Cargo.lock /id-generator/Cargo.lock
COPY ./Cargo.toml /id-generator/Cargo.toml
COPY ./.env /id-generator/.env

RUN cargo build --release
RUN rm src/*.rs

COPY ./src /id-generator/src

#RUN rm id-generator/target/release/deps/id-generator*
RUN cargo build --release

FROM alpine
COPY --from=builder /id-generator/target/release/id-generator .
COPY --from=builder /id-generator/.env .env

CMD ["./id-generator"]
EXPOSE 8080