import { AccessTokenError, getAccessToken } from "@auth0/nextjs-auth0"
import axios from "axios";
import { NextApiRequest, NextApiResponse } from "next";
import { EXTERNAL_GRAPHQL_API_URL } from "../../src/lib/constants";
export default async function gqlProxy(req: NextApiRequest, res: NextApiResponse) {
    if (EXTERNAL_GRAPHQL_API_URL == null) {
        res.status(500).send({ data: { fatal_errors: ["Internal Server Error: The server is not in the correct state to perform the request"] }, errors: null })
        return;
    }
    console.log(EXTERNAL_GRAPHQL_API_URL)
    if (req.method !== "POST") {
        res.status(200).send({ data: { fatal_errors: ["Invalid Request method. expecting POST"] } })
        return;
    }
    if (!req.headers["content-type"].includes("application/json")) {
        res.status(200).send({ data: { fatal_errors: [`This handler only understands 'application/json' got ${req.headers['content-type']}`] } })
    }
    if (req.body == null) {
        res.status(200).send({ data: { fatal_errors: ["Bad Request: Missing request body"] } })
        return
    }
    let accessToken = null;;
    try {
        accessToken = await getAccessToken(req, res)
    }
    catch (e) {
        if (e instanceof AccessTokenError) {
            res.status(200).send({ data: { fatal_errors: [e.toString()], errors: null } });
            return
        }
        res.status(200).send({ data: { fatal_errors: [e.toString()], errors: null } })
    }
    let proxyResponse = await axios.post(EXTERNAL_GRAPHQL_API_URL, req.body, { withCredentials: true, headers: { ...req.headers as any, "authorization": `Bearer ${accessToken?.token}` } });
    res.status(proxyResponse.status)
    for( let [k,v] of Object.entries(proxyResponse.headers) ) {
        res.setHeader(k,v)
    }
    res.send(proxyResponse.data)
}

export const config = {
    externalResolver: true
}