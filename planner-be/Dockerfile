FROM --platform=linux/amd64 lukemathwalker/cargo-chef:latest-rust-alpine3.18 as chef
WORKDIR /app

FROM chef AS planner
RUN apk add --update libpq
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
RUN apk add --update libpq
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN  cargo build --release

FROM --platform=linux/amd64 alpine:3.18
RUN apk add --update libpq
COPY --from=builder /app/target/release/planner-be /
CMD ["./planner-be"]
