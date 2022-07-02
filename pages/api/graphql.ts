// import type { NextApiResponse, NextApiRequest } from "next"
import "reflect-metadata";
// import { resolvers, applyResolversEnhanceMap } from "@generated/type-graphql";
import fs from "fs";
import path from "path";
// import type { ResolversEnhanceMap } from "@generated/type-graphql";
// import {
//   Authorized,
//   buildSchema,
//   buildTypeDefsAndResolvers,
// } from "type-graphql";

import {console} from "../../lib/logger"
// import { ApolloServer } from "apollo-server-micro";
// import { makeExecutableSchema } from "@graphql-tools/schema";
import { graphql, printSchema } from "graphql";
// import corsAsync from "cors-async";
import { NextApiRequest, NextApiResponse } from "next";

// import the prisma ORM client
// import prisma from "../../lib/prisma";
import { unstable_getServerSession } from "next-auth";
// import { ROLE_PREFIX, SCOPE_PREFIX } from "../../lib/rbac";
import { parseAuthenticationHeader } from "../../lib/http";
import { authOptions } from "../../lib/nextAuth/index.js";

// const resolversEnhanceMap: ResolversEnhanceMap = {
//   Address: {
//     deleteAddress: [Authorized("scope:address:delete")],
//   },
//   AccessCode: {
//     deleteAccessCode: [Authorized("scope:access_code:delete")],
//   },
// };
// applyResolversEnhanceMap(resolversEnhanceMap);
// const schema = await buildSchema({
//   resolvers,
//   validate: false,
//   authChecker: ({ root, args, context, info }, roles) => {
//     // we are abusing the 'roles' feature to allow us to inject required scopes
//     let requiredScopes = roles
//       .filter((r) => typeof r === "string")
//       .filter((r) => r.startsWith(SCOPE_PREFIX));
//     let allowedRoles = roles
//       .filter((r) => typeof r === "string")
//       .filter((r) => r.startsWith(ROLE_PREFIX));
//     // any resolver that is protected with authentication will be evaluated as follows

//     // if required scopes length is >= 0 then the user must have those scopes assigned to them,
//     // or must be assigned a role which contains those required scopes.
//     // if the user does not have the required scopes, but is assigned to a group that is allowed to perform the action,
//     // deny permission. the user must be assinged one or more of the given roles that satisfy the permission requirements,
//     // but must also possess the permissions needed to perform the action.
//     // This is for special cases where a user can be a 'ADMIN' but should not be able to perform a protected action
//     // console.log(roles)
//     // console.log(root,args,context,info)
//     return false;
//   },
// });
// export the schema file if not in production
// if (process.env.NODE_ENV !== "production") {
//   fs.writeFileSync(
//     path.resolve(process.cwd(), "schema.graphql"),
//     printSchema(schema),
//     { flag: "w" }
//   );
// }
// this configures this nextjs api route
export const config = {
  api: {
    bodyParser: true,
  },
};

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

  res.status(501).send("Under maintainence, Api Not available");
  return;

  if (req.method === "POST" && req.body == null) {
    res.status(400).send({ errors: ["Missing request body"], data: null });
    return;
  }
  if (req.method == "POST") {
    // console.dir(req.body);
    // build up the graphql execution context
    let token = null;
    // if an authentication token is provided, detect and extract the token
    let authentication = parseAuthenticationHeader(req.headers.authorization);
    let session = null;
    // using next-auth, try to  get the session from next-auth
    try {
      session = await unstable_getServerSession(req, res, authOptions);
    } catch (getSessionError) {
      console.warn(
        "Something went wrong while trying to call unstable_getServerSession. session will be null server side"
      );
      console.error(getSessionError);
    }
    console.info("Finished GetSession, session is", typeof session);
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
        contextValue: { prisma },
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
