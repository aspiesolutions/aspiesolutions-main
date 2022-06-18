import {createClient,gql} from "urql"

const defaultUrl = "/api/graphql"

export const client = createClient({
    url: process.env.GRAPHQL_URL || defaultUrl
})

export const getStatesQuery = gql`
  query getStatesQuery {
    states {
      id
      name
      abbreviation
    }
  }
`;