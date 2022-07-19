from node:16-buster as base
WORKDIR /app
# install node
# Using Debian, as root
COPY . .
RUN yarn install
RUN yarn build


FROM node:16-buster as runtime
WORKDIR /app
ENV DATABASE_URL=
COPY --from=base /app/node_modules/ /app/node_modules/
COPY --from=base /app/yarn.lock /app/package.json /app/
COPY --from=base /app/.next /app/.next
ENTRYPOINT [ "yarn", "start" ]