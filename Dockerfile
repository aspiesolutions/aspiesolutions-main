from debian:buster as base
WORKDIR /app
run apt-get update
# install deps
ARG DEBIAN_FRONTEND="noninteractive"
RUN export DEBIAN_FRONTEND=${DEBIAN_FRONTEND}
RUN apt-get install  build-essential curl autoconf automake libtool pkg-config git -y
# install node
# Using Debian, as root
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g npm yarn
COPY . .
RUN yarn install
RUN yarn build
ENTRYPOINT [ "yarn", "start" ]