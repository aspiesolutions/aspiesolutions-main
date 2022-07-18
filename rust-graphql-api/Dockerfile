
FROM debian:stable-slim AS runtime
WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/debug/rust-graphql-api /usr/local/bin
COPY .Rocket.toml /app/Rocket.toml
ENV ROCKET_DATABASES="{default={url=${DATABASE_URL}}}"
EXPOSE 65535
ENV PORT=65535
ENV ROCKET_PORT=${PORT}
ENTRYPOINT ["/usr/local/bin/rust-graphql-api"]