// import { Provider as UrqlProvider } from "urql";
import { AppProps } from "next/app";
import { RelayEnvironmentProvider } from "react-relay/hooks";
import {getInitialPreloadedQuery,getRelayProps} from 'relay-nextjs/app'
import {getClientEnvironment} from "../src/lib/relay-nextjs/clientEnvironment"
import { UserProvider } from '@auth0/nextjs-auth0';
import "../styles/globals.css";
const clientEnv = getClientEnvironment();
const initialPreloadedQuery = getInitialPreloadedQuery({
  createClientEnvironment: () => getClientEnvironment()!,
});
function MyApp({ Component, pageProps }) {
  const relayProps = getRelayProps(pageProps, initialPreloadedQuery);
  const env = relayProps.preloadedQuery?.environment ?? clientEnv!;
  return (
    <UserProvider>

      <RelayEnvironmentProvider environment={env} >
      <Component {...pageProps} {...relayProps} />
      </RelayEnvironmentProvider>
    </UserProvider>
  );
}

export default MyApp;
