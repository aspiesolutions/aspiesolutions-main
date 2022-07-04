import { printSchema } from "graphql"
import "reflect-metadata"
import buildSchema from "../../../src/lib/graphql"


export default async function generateSchema(req,res) {
    if(req.method === "OPTIONS") {
        // cors headers
    }
    else if (req.method !== "GET") {
        res.status(405).send(null)
        return
    }
    res.setHeader('content-type','text/plain; charset=utf-8')
    let schema = await buildSchema()
    let schemaString = printSchema(schema)
    res.send(schemaString)
}