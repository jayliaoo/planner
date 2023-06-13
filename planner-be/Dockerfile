FROM --platform=linux/amd64 rust:latest as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM --platform=linux/amd64 rust:latest as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM --platform=linux/amd64 rust:latest as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN  cargo build --release

FROM --platform=linux/amd64 gcr.io/distroless/cc
COPY --from=builder /app/target/release/planner-be /
CMD ["./planner-be"]
