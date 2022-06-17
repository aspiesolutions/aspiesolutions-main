// import type { NextApiResponse, NextApiRequest } from "next"
import "reflect-metadata";
import { resolvers } from "@generated/type-graphql";
import fs from "fs";
import path from "path";
import { buildSchema, buildTypeDefsAndResolvers } from "type-graphql";
// import { ApolloServer } from "apollo-server-micro";
// import { makeExecutableSchema } from "@graphql-tools/schema";
import { graphql, printSchema } from "graphql";
// import corsAsync from "cors-async";
import { NextApiRequest, NextApiResponse } from "next";

import { PrismaClient } from "@prisma/client";


const schema = await buildSchema({
  resolvers,
  validate: false,
});
// export the schema file if not in production
if (process.env.NODE_ENV !== "production") {
  fs.writeFileSync(
    path.resolve(process.cwd(), "schema.graphql"),
    printSchema(schema),
    { flag: "w" }
  );
}
// this configures this nextjs api route
export const config = {
  api: {
    bodyParser: true,
  },
};
// create the prisma ORM client
const prisma = new PrismaClient()
export default async function graphqlHandler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  res.setHeader("access-control-allow-origin", "*");
  res.setHeader("access-control-allow-headers", "content-type");
  if (req.method === "OPTIONS") {
    res.status(200);
    res.send(null);
    return;
  }

  if (req.method === "POST" && req.body == null) {
    res.status(400).send("expected a body in request");
    return;
  }
  if (req.method == "POST") {
    console.dir(req.body);
    // extract query from body if its a string, else extract it from query object on body, else null
    let query =
      typeof req.body === "string" ? req.body : req?.body?.query || null;
    // extract query variables if on body as variables, otherwise null
    let variables = req?.body?.variables || null;
    let operationName = req?.body?.operationName || null;

    try {
      let executionResult = await graphql({
        schema,
        source: typeof req.body === "string" ? req.body : req.body?.query,
        variableValues: variables,
        contextValue: {prisma},
        operationName,
      });
      if (executionResult.errors) {
        res.status(400);
      } else {
        res.status(200);
      }
      res.setHeader("content-type", "application/json");
      res.send(JSON.stringify(executionResult));
    } catch (executionError) {
      console.error(executionError);
      res.status(500).send(null);
      return;
    }
  }
  res.status(405).send(null);

  // res.setHeader("access-control-allow-methods","GET, POST, OPTIONS")
  // res.setHeader("access-control-allow-headers","content-type")
  // // res.setHeader("allow","GET, POST, OPTIONS")
  // return handler(req,res)
}
