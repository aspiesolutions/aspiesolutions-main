from debian:buster as base
WORKDIR /app
run apt-get update
run apt-get upgrade -y
# install deps
ARG DEBIAN_FRONTEND="noninteractive"
RUN export DEBIAN_FRONTEND=${DEBIAN_FRONTEND}
RUN apt-get install  build-essential curl autoconf automake libtool pkg-config git -y
# install node
# Using Debian, as root
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g npm yarn
# build libpostal
RUN git clone https://github.com/openvenues/libpostal
WORKDIR /app/libpostal
RUN ./bootstrap.sh && ./configure --datadir="$PWD/data" && make -j && make install && ldconfig
WORKDIR /app
COPY . .
RUN yarn install
RUN yarn build