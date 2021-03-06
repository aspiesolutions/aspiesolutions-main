import { Provider as UrqlProvider } from "urql";
import "../styles/globals.css";
import {client} from "../lib/urql"
function MyApp({ Component, pageProps }) {
  return (
    <UrqlProvider value={client}>
      <Component {...pageProps} />
    </UrqlProvider>
  );
}

export default MyApp;
