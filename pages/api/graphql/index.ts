import "reflect-metadata";
import fs from "fs";
import path from "path";
import { execute} from "graphql";
// import corsAsync from "cors-async";
import { NextApiRequest, NextApiResponse } from "next";

// import { unstable_getServerSession } from "next-auth";
// import { parseAuthenticationHeader } from "../../src/lib/http";
// import { authOptions } from "../../src/lib/nextAuth/index.js";

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
