// import type { NextApiResponse, NextApiRequest } from "next"
import "reflect-metadata";
import { resolvers } from "@generated/type-graphql";
import { buildSchema, buildTypeDefsAndResolvers } from "type-graphql";
import { ApolloServer } from "apollo-server-micro";
import { makeExecutableSchema } from "@graphql-tools/schema";
import { graphql, printSchema } from "graphql";
import corsAsync from "cors-async";
import { NextApiRequest, NextApiResponse } from "next";

const schema = await buildSchema({
  resolvers,
  validate: false,
});
// const schema = makeExecutableSchema({typeDefs,resolvers})
// const apolloServer = new ApolloServer( {
//     schema,
//     cors: false
// })
export const config = {
  api: {
    bodyParser: true,
  },
};

// let s = await apolloServer.start()
// let handler = await apolloServer.createHandler({path:"/api/graphql",})
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

    try {
      let executionResult = await graphql({ schema, source: typeof req.body === "string" ? req.body : req.body?.query });
      if (executionResult.errors) {
        res.status(400);
      } else {
        res.status(200);
        res.setHeader("content-type","application/json");
        res.send(JSON.stringify(executionResult))
      }
      res.send(null);
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
