FROM rustlang/rust:nightly-slim

WORKDIR "/app"

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src ./src

RUN cargo fetch
RUN cargo build --release

CMD ["cargo", "run"]
