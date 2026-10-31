# syntax=docker/dockerfile:1

# 1. Rust toolbox with cargo-chef, which caches dependencies between builds
FROM lukemathwalker/cargo-chef:latest-rust-1.99.1 AS chef
WORKDIR /src

# 2. Make a "shopping list" of the dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 3. Build the dependencies, then code
FROM chef AS builder
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin api --bin bot

# 4. Final images
FROM gcr.io/distroless/cc-debian13:nonroot AS api
COPY --from=builder /src/target/release/api /usr/local/bin/api
ENV API_ADDR=0.0.0.0:5080
EXPOSE 5080
ENTRYPOINT [ "/usr/local/bin/api" ]

FROM gcr.io/distroless/cc-debian13:nonroot AS bot
COPY --from=builder /src/target/release/bot /usr/local/bin/bot
ENTRYPOINT ["/usr/local/bin/bot"]
