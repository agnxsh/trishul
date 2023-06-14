FROM rust:1.70

WORKDIR /trishul

COPY . .

RUN apt update

RUN cargo build