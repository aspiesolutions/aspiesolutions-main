import { NextApiRequest, NextApiResponse } from "next";
import "../../lib/graphql"


export default async function graphqlHandler(request:NextApiRequest,res:NextApiResponse) {
    res.status(200).send(null)
}