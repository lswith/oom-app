FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef

FROM chef AS planner
COPY . /rust
WORKDIR /rust
ENV RUSTFLAGS="-Dwarnings"
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS deps 
COPY --from=planner /rust/recipe.json /rust/recipe.json
WORKDIR /rust
ENV RUSTFLAGS="-Dwarnings"
RUN cargo chef cook --release --recipe-path recipe.json

FROM deps AS builder
COPY . /rust
ENV CARGO_BUILD_RUSTFLAGS="-Dwarnings"
RUN cargo build --release 
RUN cargo install --profile release --path .

CMD ["/usr/local/cargo/bin/oom-app"]
