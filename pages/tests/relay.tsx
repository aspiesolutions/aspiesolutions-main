// src/pages/user/[uuid].tsx
import { withRelay, RelayProps } from 'relay-nextjs';
import { graphql, usePreloadedQuery } from 'react-relay/hooks';
import { getClientEnvironment } from '../../src/lib/relay-nextjs/clientEnvironment';
import { relay_AccessCodeQuery } from '../../src/queries/__generated__/relay_AccessCodeQuery.graphql';
import { createServerEnvironment } from '../../src/lib/server/relay-nextjs/serverEnvironment';
// The $uuid variable is injected automatically from the route.
const AccessCodeQuery = graphql`
  query relay_AccessCodeQuery {
    accessCode {
      id
    }
  }
`;

function AccessCode({ preloadedQuery }: RelayProps<{}, relay_AccessCodeQuery>) {
  const query = usePreloadedQuery(AccessCodeQuery, preloadedQuery);

  return (
    <>Hello {query.accessCode}</>
  );
}

function Loading() {
  return <div>Loading...</div>;
}

export default withRelay(AccessCode, AccessCodeQuery, {
  // Fallback to render while the page is loading.
  // This property is optional.
  fallback: <Loading />,
  // Create a Relay environment on the client-side.
  // Note: This function must always return the same value.
  createClientEnvironment: () => getClientEnvironment()!,
  // Gets server side props for the page.
  serverSideProps: async (ctx) => {
    // This is an example of getting an auth token from the request context.
    // If you don't need to authenticate users this can be removed and return an
    // empty object instead.
    // const { getTokenFromCtx } = await import('lib/server/auth');
    // const token = await getTokenFromCtx(ctx);
    // if (token == null) {
    //   return {
    //     redirect: { destination: '/login', permanent: false },
    //   };
    // }

    return {};
  },
  // Server-side props can be accessed as the second argument
  // to this function.
  createServerEnvironment: async (
    ctx,
    // The object returned from serverSideProps. If you don't need a token
    // you can remove this argument.
    // { token }: { token: string }
  ) => {
    return createServerEnvironment();
  },
});