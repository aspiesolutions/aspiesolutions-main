import Head from "next/head";
// import Image from 'next/image'
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import "@mapbox/mapbox-gl-geocoder/dist/mapbox-gl-geocoder.css";
import MapboxGeocoder from "@mapbox/mapbox-gl-geocoder";
import Link from "next/link";
import { User } from "../src/lib/graphql/models/User";
import { NEXTAUTH_DEFAULT_PROVIDER } from "../src/lib/constants";

// import { Address } from '@universe/address-parser/esm/src/index'
import { useCallback, useEffect, useRef, useState } from "react";
import styles from "../styles/Home.module.css";
import { gql, useMutation, useQuery } from "urql";
import { useRouter } from "next/router";
import { NextAuthOptions, unstable_getServerSession } from "next-auth";
import { getSession, signIn } from "next-auth/react";
import {
  REQUIRED_SCOPE_MISSING,
  UnauthorizedDatabaseTransaction,
  UNAUTHORIZED_PRISMA_TRANSACTION,
} from "../src/lib/rbac";

/// types definition for this mini app
type ServerSideError = {
  kind;
  reason;
  action;
  message;
};
type AccessCodeError = {
  kind;
  reason;
  action;
  message;
};
type AccessCodeProps = {
  error: null;
  data: null;
};
type AddressData = {
  query;
  situs;
  uspsLabel;
  matches;
};
type AddressProps = {
  error: AccessCodeError | null;
  data: AddressData | null;
};
type ServerSideProps = {
  error: ServerSideError | null;
  session: null | any,
  address: AddressProps;
  accessCodes: AccessCodeProps;
};

const defaultMapboxToken =
  "pk.eyJ1IjoianRoZWN5YmVydGlua2VyZXIiLCJhIjoiY2w0bjRicWFzMWs2eTNpcGd5c2UyYm1tbCJ9.gtMxHjwKheor-JFsyfx19g";
mapboxgl.accessToken = defaultMapboxToken;

const addressQuery = gql`
  query getFirstAddressQuery($address: String!) {
    findFirstAddress(where: { text: { equals: $address } }) {
      text
      access_codes {
        value
      }
    }
  }
  # query getClosestAddressesQuery {

  # }
`;

const addAddressWithDefaultsMutation = gql`
  mutation addAddressWithDefaults($text: String!) {
    createAddress(data: { text: $text }) {
      text
    }
  }
`;
// USED TO DEFINE AND HANDLE ERRORS
const ERROR_UNAUTHORIZED = "UNAUTHORIZED";
const NO_ERROR = null;
const ERROR_GENERAL_FAILURE = "ERROR_GENERAL_FAILURE";

const REASON_NO_ERROR = "NO_ERROR_DETECTED";
const REASON_NULL_SESSION = "NULL_SESSION";
const REASON_UNKNOWN = "REASON_UNKNOWN";
// the server has instructed the client to attempt to initate the authentication flow.
// the client should be programed to redirect to the authentication page at this point

const MESSAGE_NO_ERROR = `No error detected`;
const MESSAGE_SERVER_GENERAL_FAILURE = `Server returned an error. The application is unable to determine the kind of error that occurred.
Please contact the administrator. Unable to continue`;
const MESSAGE_AUTHENTICATON_ATTEMPT_REQUIRED =
  "The server requires you to attempt authentication before continuing";

const ACTION_ATTEMPT_AUTHENTICATION = "ATTEMPT_AUTHENTICATION";
const ACTION_HALT = "ACTION_HALT";
const ACTION_CONTINUE = "ACTION_CONTINUE";
const ACTION_DEFAULT_NO_ERROR = "ACTION_DEFAULT_CONTINUE";

function AccessCodes(props) {
  switch (props?.error?.kind) {
    case UNAUTHORIZED_PRISMA_TRANSACTION:
      return props?.error?.message || "Database oops";
      break;
  }
  return <>{"hello from accessCodes"}</>;
}

export default function Home(props) {
  // check for geolocation
  const router = useRouter();
  // perform initial check for a server side error
  const mapboxContainer = useRef(null);
  const map = useRef(null);
  const [lat, setLat] = useState(-98.8223185136653);
  const [lng, setLng] = useState(31.8039734986);
  const [mapboxAddressResult, setMapboxAddress] = useState(null);
  const [zoom, setZoom] = useState(5);
  // const [gqlAddressQueryResult, reexecuteAddressQuery] = useQuery({
  //   query: addressQuery,
  //   variables: { address: mapboxAddressResult?.result?.place_name || "" },
  // });
  // const [gqlCreateAddressMutationResult,createAddress] = useMutation(addAddressWithDefaultsMutation)
  useEffect(() => {
    if (props?.error?.kind != NO_ERROR || map.current) return;
    map.current = new mapboxgl.Map({
      container: mapboxContainer?.current,
      style: "mapbox://styles/mapbox/streets-v11",
      center: [lat, lng],
      zoom,
    });
    map.current.addControl(
      new mapboxgl.GeolocateControl({
        positionOptions: {
          enableHighAccuracy: true,
        },
        trackUserLocation: true,
        showUserHeading: true,
      })
    );
    let geocoder = new MapboxGeocoder({
      accessToken: mapboxgl.accessToken,
      mapboxgl: mapboxgl,
    });
    geocoder.on("result", (selected) => {
      console.log("selected location");
      console.dir(selected);
      setMapboxAddress(selected);
    });
    map.current.addControl(geocoder);
  });

  useEffect(() => {
    if (props?.error?.kind !== NO_ERROR || !map.current) return;
    map.current.on("move", () => {
      setLng(map.current.getCenter().lng.toFixed(16));
      setLat(map.current.getCenter().lat.toFixed(16));
      setZoom(map.current.getZoom());
    });
  });
  useEffect(() => {
    // dont perform any action when an error happens
    if (props?.error?.kind !== NO_ERROR) {
      return;
    }
    // update the query params when the user types in an address
    if (!router.isReady || mapboxAddressResult == null) return;
    // we have to check if the address matches, otherwise an infinite update loop occurs
    console.log("update query useffect");
    if (router.query.address == mapboxAddressResult?.result?.place_name) return;
    router.replace({
      pathname: router.pathname,
      query: {
        ...router.query,
        address: mapboxAddressResult?.result?.place_name,
      },
    });
  }, [props?.error?.kind, router, mapboxAddressResult]);
  // if the server returns an error for an unknown reason, just print its message
  if (props?.error?.kind === ERROR_GENERAL_FAILURE) {
    return (
      <div>
        {props?.error?.message} &quot;{props?.error?.reason}&quot;
      </div>
    );
  }
  if(props?.session == null) {
    return (
      <div>
        <p>{props?.error?.message}</p>
        <Link href="/api/auth/signin">
          <span
            onClick={(e) => {
              e.preventDefault();
              signIn(NEXTAUTH_DEFAULT_PROVIDER);
            }}
          >
            Please click here to attempt to log in
          </span>
        </Link>
      </div>
    );
  }
  return (
    <div className={styles.container}>
      <Head>
        <title>Gatekeeper</title>
        <meta name="description" content="Track Access Codes" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <div>
          {zoom} {lat}, {lng}
        </div>
        <div
          id="mapbox-container"
          style={{ minHeight: "200px" }}
          ref={mapboxContainer}
        ></div>
        <div>current Address {mapboxAddressResult?.result?.place_name}</div>
        {/* display the access codes here */}
        <AccessCodes
          error={props?.accessCodes?.error}
          data={props?.accessCodes?.data}
        />
      </main>
    </div>
  );
}

export async function getServerSideProps(context) {
  // early initialize data structures
  let initialProps: ServerSideProps = {
    error: null,
    session: null,
    address: {
      data: null,
      error: null,
    },
    accessCodes: {
      data: null,
      error: null,
    },
  };
  // always check the session first. this mini-app deals with sensitive data that should not be publicly available
  const { authOptions } = require("../src/lib/nextAuth");
  let session = null;
  try {
    console.log("trying unstable_getServerSession")
    session = await unstable_getServerSession(
      context?.req,
      context?.res,
      authOptions
    );
  } catch (error) {
    console.error(
      "unstable_getServerSessionFailed. falling back to getSession"
    );
  }
  if (session == null) {
    // try again using getSession
    console.log("trying again with getSession")
    try {
      session = await getSession({ req: context.req });
    } catch (error) {
      console.error(
        "getSessionFailed. getting the server side session is unavailable unless another authentication method can be devised"
      );
      console.error(error)
    }
  }
  initialProps.session = session;
  // this is a client factory

  if (initialProps.session == null) {
    context.res.statusCode = 403;
    initialProps.error = {
      kind: ERROR_UNAUTHORIZED,
      reason: REASON_NULL_SESSION,
      action: ACTION_ATTEMPT_AUTHENTICATION,
      message: MESSAGE_AUTHENTICATON_ATTEMPT_REQUIRED,
    };

  }
  return {
    props: initialProps,
  };
  // ORM is not set up yet. moving from prisma to typeorm
}
