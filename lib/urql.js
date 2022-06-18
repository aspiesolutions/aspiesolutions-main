import {createClient} from "urql"

const defaultUrl = "/api/graphql"

export const client = createClient({
    url: process.env.GRAPHQL_URL || defaultUrl
})