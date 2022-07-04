import { Provider as UrqlProvider } from "urql";
import { AppProps } from "next/app";
import { RelayEnvironmentProvider } from "react-relay/hooks";
import {getInitialPreloadedQuery,getRelayProps} from 'relay-nextjs/app'
import {getClientEnvironment} from "../src/lib/relay-nextjs/clientEnvironment"
import "../styles/globals.css";
import {client} from "../src/lib/urql"
const clientEnv = getClientEnvironment();
const initialPreloadedQuery = getInitialPreloadedQuery({
  createClientEnvironment: () => getClientEnvironment()!,
});
function MyApp({ Component, pageProps }) {
  const relayProps = getRelayProps(pageProps, initialPreloadedQuery);
  const env = relayProps.preloadedQuery?.environment ?? clientEnv!;
  return (
    <UrqlProvider value={client}>
      <RelayEnvironmentProvider environment={env} >
      <Component {...pageProps} {...relayProps} />
      </RelayEnvironmentProvider>
    </UrqlProvider>
  );
}

export default MyApp;
