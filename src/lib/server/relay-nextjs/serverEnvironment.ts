import { graphql } from 'graphql';
import { withHydrateDatetime } from 'relay-nextjs/date';
import { Environment, GraphQLResponse, Network, RecordSource, Store } from 'relay-runtime';
// import buildSchema from '../graphql';
// import { AppDataSource } from '../typeorm/data-source';
// Relay is not prescriptive about how GraphQL requests are made.
// This is an example showing how to request GraphQL data.
// You should fill this in with how to make requests to your GraphQL
// API of choice.
import axios from "axios"
import buildSchema from "../../graphql"
import { AppDataSource } from '../../data-source';
const DEFAULT_URL = "http://localhost:3000"

// eventually we will need to build in the authentication logic here.
export function createServerNetwork() {
  let datasourceInitializer = AppDataSource.initialize()
  let schemaInitializer =buildSchema();
  return Network.create(async (params, variables) => {
    let datasource = await datasourceInitializer;
    let schema = await schemaInitializer;
    let executionResult = await graphql({ schema, source: params.text, variableValues: variables, contextValue: { datasource }, operationName: params.name })

    const data = JSON.parse(
      JSON.stringify(executionResult),
      withHydrateDatetime
    ) as GraphQLResponse;

    return data;
  });
}

// Optional: this function can take a token used for authentication and pass it into `createServerNetwork`.
export function createServerEnvironment() {
  return new Environment({
    network: createServerNetwork(),
    store: new Store(new RecordSource()),
    isServer: true,
  });
}