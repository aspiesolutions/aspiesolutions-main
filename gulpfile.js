require("dotenv").config();
const GulpClient = require("gulp");
const gulp = require("gulp");
const childProcess = require("node:child_process");
const path = require("path");
let cargoWatchApiServer = null;
gulp.task("start-dev", async () => {
  gulp.parallel("watch-lib-graphql", "watch-rocket", "next-dev");
});
gulp.task("watch-lib-graphql", (done) => {
  let watcher = gulp
    .watch("./lib-graphql/")
    .on("change", () => {
      let child = childProcess.exec(
        "cargo",
        ["run", "--release", "--bin", "output-schema"],
        {
          env: { ...process.env },
        }
      );
      child.stdout.pipe(process.stdout);
      watcher.on("close", () => {
        child.kill();
      });
    })
    .on("close", done);

  process.on("exit", () => {
    watcher.close();
  });
});

gulp.task("watch-rocket", (done) => {
  cargoWatchApiServer = childProcess.spawn("cargo-watch", [
    `-w`,
    path.resolve(__dirname, "rust-graphql-api"),
    "-w",
    path.resolve(__dirname, "entity"),
    "-w",
    path.resolve(__dirname, "migration"),
    "-w",
    path.resolve(__dirname, "aspiesolutions_core"),
    "-x check --package rust-graphql-api",
    "-x test -- package rust-graphql-pi",
    "-x run --bin rust-graphql-api",
  ]);
  cargoWatchApiServer.stderr.pipe(process.stderr);
  cargoWatchApiServer.stdout.pause(process.stdout);
  cargoWatchApiServer.on("close", done);
  cargoWatchApiServer.on("exit", done);
  process.on("SIGHUP", () => {
    cargoWatchApiServer.kill("SIGHUP");
  });
});
gulp.task("next-dev", (done) => {
  let child = childProcess.spawn(
    path.resolve(__dirname, "node_modules", ".bin", "next"),
    ["dev"]
  );
  child.stdout.pipe(process.stdout);
  child.stderr.pipe(process.stderr);
  child.on("exit", done);
  child.on("error", done);
  child.on("close", done);
});
GulpClient.task("relay-compiler");
