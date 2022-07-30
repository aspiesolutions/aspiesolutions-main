// lib/client_environment.ts
import { getRelaySerializedState } from "relay-nextjs";
import { withHydrateDatetime } from "relay-nextjs/date";
import { Environment, Network, Store, RecordSource } from "relay-runtime";

const API_URL =
  process.env.NODE_ENV === "production"
    ? "/api/graphql"
    : "http://localhost:65535/api/graphql";

export function createClientNetwork() {
  return Network.create(async (params, variables) => {
    // relay crashes the program when you return "errors" with no data.
    // transform the "errors" into data.errors so the client can respond to them
    let relay_response = { data: null, errors: null };
    let response = null;
    try {
      response = await fetch(API_URL, {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
          authorization: "Bearer someinvalidtoken",
        },
        body: JSON.stringify({
          query: params.text,
          variables,
        }),
      });
    } catch (e) {
      console.error(e);
      relay_response.data = { errors: [e?.toString()] };
      return relay_response;
    }
    return JSON.parse(await response.text(), withHydrateDatetime);
  });
}

let clientEnv: Environment | undefined;
export function getClientEnvironment() {
  if (typeof window === "undefined") return null;

  if (clientEnv == null) {
    clientEnv = new Environment({
      network: createClientNetwork(),
      store: new Store(new RecordSource(getRelaySerializedState()?.records)),
      isServer: false,
    });
  }

  return clientEnv;
}
