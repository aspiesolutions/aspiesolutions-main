// src/pages/user/[uuid].tsx
import { withRelay, RelayProps } from "relay-nextjs";
import { graphql, usePreloadedQuery } from "react-relay/hooks";
import { getClientEnvironment } from "../../src/lib/relay-nextjs/clientEnvironment";
import { relay_AccessCodeQuery } from "../../src/queries/__generated__/relay_AccessCodeQuery.graphql";
// import { createServerEnvironment } from '../../src/lib/server/relay-nextjs/serverEnvironment';
import { NextApiRequest } from "next";
import { withPageAuthRequired } from "@auth0/nextjs-auth0";

// The $uuid variable is injected automatically from the route.
const AccessCodeQuery = graphql`
  query relay_AccessCodeQuery {
    accessCode(id: "abcdefg") {
      id
    }
  }
`;

function AccessCode({ preloadedQuery }: RelayProps<{}, relay_AccessCodeQuery>) {
  const query = usePreloadedQuery(AccessCodeQuery, preloadedQuery);
  if (!query.accessCode && query.errors) {
    return <>No access code returned because the request failed</>;
  } else if (!query.accessCode) {
    return <>no access code was returned because it was not found</>;
  }
  return <>Hello {query.accessCode.id}</>;
}

function Loading() {
  return <div>Loading...</div>;
}

export default withRelay(withPageAuthRequired(AccessCode), AccessCodeQuery, {
  // Fallback to render while the page is loading.
  // This property is optional.
  fallback: <Loading />,
  // Create a Relay environment on the client-side.
  // Note: This function must always return the same value.
  createClientEnvironment: () => {
    return getClientEnvironment()!;
  },
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
    let req = ctx.req as NextApiRequest;
    let { getAccessToken, AccessTokenError } = await import(
      "@auth0/nextjs-auth0"
    );
    // let {authOptions} = await import("../../src/lib/nextAuth/index")
    let getTokenResult = null;
    try {
      getTokenResult = await getAccessToken(ctx.req, ctx.res, { scopes: [] });
    } catch (e) {
      if (e instanceof AccessTokenError && e.code == "invalid_session") {
        console.info("user not logged in or session expired");
        ctx.res.statusCode = 307;
        ctx.res.setHeader("location", `/api/auth/login`);
        ctx.res.end();
        return;
      } else {
        console.warn(
          "Could not handle an invalid token error. redirecting user to homepage"
        );
        ctx.res.statusCode = 307;
        ctx.res.setHeader("location", "/");
        ctx.res.end();
        return;
      }
    }

    console.log("assembled token");
    return { token: getTokenResult?.accessToken };
  },
  // Server-side props can be accessed as the second argument
  // to this function.
  createServerEnvironment: async (
    ctx,
    // The object returned from serverSideProps. If you don't need a token
    // you can remove this argument.
    { token }: { token: string }
  ) => {
    // withApiAuthRequired()
    return (
      await import("../../src/lib/server/relay-nextjs/serverEnvironment")
    ).createServerEnvironment(token);
  },
});
