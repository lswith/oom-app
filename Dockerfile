FROM rust:1.69

WORKDIR /usr/src/oom-app
COPY . .

RUN cargo install --path .

CMD ["oom-app"]
