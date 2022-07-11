from debian:buster as base
WORKDIR /app
run apt-get update
run apt-get upgrade -y
# install deps
ARG DEBIAN_FRONTEND="noninteractive"
ENV CARGO_HOME=".cargo/bin"
ENV PATH="${PATH}:${CARGO_HOME}"
ENV CARGO = "~/.cargo/bin/cargo"
ENV RUSTUP = "~/.cargo/bin/rustup"
RUN export DEBIAN_FRONTEND=${DEBIAN_FRONTEND}
RUN apt-get install  build-essential curl autoconf automake libtool pkg-config git -y
# install node
# Using Debian, as root
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g npm yarn
# build libpostal
RUN git clone https://github.com/openvenues/libpostal --depth 1
WORKDIR /app/libpostal
RUN ./bootstrap.sh && ./configure --datadir="$PWD" && make -j 2 && make install && ldconfig && make distclean
WORKDIR /app
# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs --output rustup.sh && chmod +x rustup.sh && ./rustup.sh -y
RUN ~/.cargo/bin/cargo install cargo-chef sea-orm-cli
WORKDIR /app
COPY . .
RUN yarn install
RUN yarn build