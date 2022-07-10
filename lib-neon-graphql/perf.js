let m = require('.')
let args = {}
args.source = `query testQuery {node(id:"abcd")}`
let then = Date.now()
let rounds = 300_000;
console.log(`running ${rounds} times`)
for(let i = 0; i < rounds; i++) {
    m.execute_juniper(args)
}
let now = Date.now()
console.log(`execution took ${now - then}`)