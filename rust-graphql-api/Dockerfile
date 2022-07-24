
FROM debian:stable AS runtime
WORKDIR /app
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install apt-utils
RUN  apt-get install ca-certificates -y
COPY ./target/debug/rust-graphql-api /usr/local/bin
COPY ./Rocket.toml /app/Rocket.toml
ENV ROCKET_DATABASES="{default={url=${DATABASE_URL}}}"
EXPOSE 65535
ENV PORT=65535
ENV ROCKET_PORT=${PORT}
ENV DATABASE_URL=
# allows the mount path of the graphql handlers to be controlled
# the
ENV GRAPHQL_PUBLIC_SEGMENT=
ENTRYPOINT ["/usr/local/bin/rust-graphql-api"]