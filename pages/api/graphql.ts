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

// import {console} from "../../lib/logger"
// import { ApolloServer } from "apollo-server-micro";
// import { makeExecutableSchema } from "@graphql-tools/schema";
import { execute} from "graphql";
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
    bodyParser: false,
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
  // this handler is written in a way to allow the client to send compressed bodies.
  // currently no compressed bodies will be handled correctly
  // this handler responds either in compressed or uncompressed json format
  let body: Buffer | null = null;
  let query = null;
  let contentType:string = req.headers['content-type'] || "text/plain";
  let contentEncoding = (req.headers['content-encoding'] || "").trim().split(", ").filter(s=>s.length > 0);
  let charSet: BufferEncoding = "utf-8"
  req.on('data',(chunk)=>{
    if (body == null) {
      body = chunk
    }
    else {
      body = Buffer.concat([body,chunk])
    }
  })
  req.on('end',()=>{
    if(contentEncoding.length > 0) {
      console.log("content encoding was provided, possible encoding was performed")
      console.warn("client request decoding is not implemented yet, this will possibly fail")
      console.log(contentEncoding)
    }
    // assume the content has been decoded from this point
    if(contentType.includes("text/plain") || contentType.includes("application/json")) {
      query = body.toString(charSet)
    }
    else {
      res.status(415).send(`Unsupported content type '${contentType}'`)
      return
    }
    // handle here
    if(req.method === "POST" && contentType.includes("application/json")) {
      console.log("client says they sent JSON content. parse it for the variables")
      console.log(query)
      try {
        let jsonBody = JSON.parse(query)
        // create the context here
        execute()
      }
      catch(error) {
        console.error(error)
        res.status(500).send(null)
      }
    }
    // detect content compression if supported and compress the content
  })
}
