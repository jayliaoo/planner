FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y --no-install-recommends libpq-dev && apt install -y --no-install-recommends libc6-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/planner-be /usr/local/bin
EXPOSE 3500
ENTRYPOINT ["/usr/local/bin/planner-be"]
