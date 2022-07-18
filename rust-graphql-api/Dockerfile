FROM lukemathwalker/cargo-chef:latest-rust-1.62 as chef
WORKDIR /app

# libpq-dev is already installed in this base image

# install cli tools

RUN cargo install sea-orm-cli

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin rust-graphql-api

FROM chef AS runtime
WORKDIR /app
RUN apt update && apt install libssl-dev -y
COPY --from=builder /app/target/release/rust-graphql-api /usr/local/bin
COPY --from=builder /app/Rocket.toml /app/Rocket.toml
ENV ROCKET_DATABASES="{default={url=${DATABASE_URL}}}"
EXPOSE 65535
ENV PORT=65535
ENV ROCKET_PORT=${PORT}
ENTRYPOINT ["/usr/local/bin/rust-graphql-api"]