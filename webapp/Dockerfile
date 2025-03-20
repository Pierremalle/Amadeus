FROM rust:1.82 as builder
WORKDIR /usr/src/myapp
COPY .. .
#RUN cargo install --path .
RUN cargo install cargo-binstall
RUN cargo binstall dioxus-cli

CMD dx serve

