const { spawn } = require("node:child_process");
const path = require("path");
// const relay_compiler = spawn("yarn", ["run", "relay-compiler", "--watch"]);
// relay_compiler.stdout.on("data", (data) => {
//   console.log(`relay-compiler: ${data}`);
// });
// relay_compiler.stderr.on("data", (error) => {
//   console.error(error.toString());
// });

// relay_compiler.on("error", (err) => {
//   console.error(err);
//   process.exit(-1);
// });
// relay_compiler.on("close", (code) => {
//   console.info("relay compiler exited with ", code);
//   process.exit();
// });
var watchman = require("fb-watchman");
var client = new watchman.Client();
client.capabilityCheck(
  { optional: [], required: ["relative_root"] },
  function (error, resp) {
    if (error) {
      // error will be an Error object if the watchman service is not
      // installed, or if any of the names listed in the `required`
      // array are not supported by the server
      console.error(error);
      return;
    }
    // resp will be an extended version response:
    // {'version': '3.8.0', 'capabilities': {'relative_root': true}}
    client.command([]);
  }
);
