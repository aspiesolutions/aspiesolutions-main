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
const DEFAULT_URL = process.env.API_URL || "http://localhost:65535/api/graphql"

// eventually we will need to build in the authentication logic here.
export function createServerNetwork(token:string) {
  return Network.create(async (params, variables) => {
    console.log("Relay network.create!");
    console.log(params);
    console.log(variables);
    console.log("token",token);
    let data = {} as any;
    try {
      let response = await axios.post(DEFAULT_URL,{query:params.text, variables},{headers:{"content-type":'application/json; charset=utf-8','authorization':`Bearer ${token}`}})
      data = response?.data;

    }
    catch (e) {
      console.error(e)
      data.errors=[e.toString()];
    }
    return data
  });
}

// Optional: this function can take a token used for authentication and pass it into `createServerNetwork`.
export function createServerEnvironment(token:string) {
  return new Environment({
    network: createServerNetwork(token),
    store: new Store(new RecordSource()),
    isServer: true,
  });
}