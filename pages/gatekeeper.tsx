import Head from "next/head";
// import Image from 'next/image'
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import "@mapbox/mapbox-gl-geocoder/dist/mapbox-gl-geocoder.css";
import MapboxGeocoder from "@mapbox/mapbox-gl-geocoder";
import Link from "next/link"

import {NEXTAUTH_DEFAULT_PROVIDER} from "../lib/constants"

// import { Address } from '@universe/address-parser/esm/src/index'
import { useCallback, useEffect, useRef, useState } from "react";
import styles from "../styles/Home.module.css";
import { gql, useMutation, useQuery } from "urql";
import { useRouter } from "next/router";
import { NextAuthOptions, unstable_getServerSession } from "next-auth";
import { signIn} from "next-auth/react"
// import { getToken } from "next-auth/jwt";
// import parseAddress from "../lib/parseAddress"
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
const ERROR_NO_ERROR = "ERROR_NO_ERROR"
const ERROR_GENERAL_FAILURE = "ERROR_GENERAL_FAILURE";

const REASON_NO_ERROR = "NO_ERROR_DETECTED"
const REASON_NULL_SESSION = "NULL_SESSION";
const REASON_UNKNOWN = "REASON_UNKNOWN";
// the server has instructed the client to attempt to initate the authentication flow.
// the client should be programed to redirect to the authentication page at this point


const MESSAGE_NO_ERROR = `No error detected`
const MESSAGE_SERVER_GENERAL_FAILURE = `Server returned an error. The application is unable to determine the kind of error that occurred.
Please contact the administrator. Unable to continue`;
const MESSAGE_AUTHENTICATON_ATTEMPT_REQUIRED =
  "The server requires you to attempt authentication before continuing";

const ACTION_ATTEMPT_AUTHENTICATION = "ATTEMPT_AUTHENTICATION";
const ACTION_HALT = "ACTION_HALT";
const ACTION_CONTINUE= "ACTION_CONTINUE"
const ACTION_DEFAULT_NO_ERROR ="ACTION_DEFAULT_CONTINUE"


function handleServerErrorFromProps(props) {
  const { error } = props;
  let defaultErrorKind = ERROR_NO_ERROR;
  let defaultErrorReason = REASON_NO_ERROR;
  let defaultErrorAction = ACTION_DEFAULT_NO_ERROR;
  let defaultErrorMessage = MESSAGE_NO_ERROR;
  if(!error) {
    return [defaultErrorKind,defaultErrorReason,defaultErrorAction,defaultErrorMessage]
  }
  let { kind, reason, action, message } = error;
  if (kind == null) {
    return [
      ERROR_GENERAL_FAILURE,
      reason || REASON_UNKNOWN,
      action || ACTION_HALT,
      message || MESSAGE_SERVER_GENERAL_FAILURE,
    ];
  }
  return [kind,reason,action,message];
}


export default function Home(props) {
  // check for geolocation
  const router = useRouter();
  // perform initial check for a server side error
  const [ errorKind, errorReason,errorAction,errorMessage ] =
    handleServerErrorFromProps(props);
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

    if (errorKind != ERROR_NO_ERROR || map.current) return;
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
    if (errorKind !== ERROR_NO_ERROR || !map.current) return;
    map.current.on("move", () => {
      setLng(map.current.getCenter().lng.toFixed(16));
      setLat(map.current.getCenter().lat.toFixed(16));
      setZoom(map.current.getZoom());
    });
  });
  useEffect(() => {
    // dont perform any action when an error happens
    if(errorKind !== ERROR_NO_ERROR) {
      return
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
  }, [errorKind,router, mapboxAddressResult]);
  // if the server returns an error for an unknown reason, just print its message
  if(errorKind === ERROR_GENERAL_FAILURE) {
    return <div>{errorMessage} "{errorReason}"</div>
  }
  if(errorKind === ERROR_UNAUTHORIZED && errorAction === ACTION_ATTEMPT_AUTHENTICATION) {
    return (<div>
      <p>{errorMessage}</p>
      <Link href="/api/auth/signin" ><span onClick={(e)=>{e.preventDefault();signIn(NEXTAUTH_DEFAULT_PROVIDER)}}>Please click here to attempt to log in</span></Link>
      </div>)
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
        {/* {mapboxAddressResult && gqlAddressQueryResult.fetching
          ? "loading"
          : null}
        {mapboxAddressResult &&
        gqlAddressQueryResult.fetching === false &&
        gqlAddressQueryResult.data?.findFirstAddress == null
          ? "This address does not exist in the databsae"
          : null}
        {mapboxAddressResult &&
        gqlAddressQueryResult.fetching === false &&
        gqlAddressQueryResult.data?.findFirstAddress?.access_codes?.length == 0
          ? "No Access Codes for this address"
          : null} */}
      </main>
    </div>
  );
}

export async function getServerSideProps(context) {
  // always check the session first. this mini-app deals with sensitive data that should not be publicly available
  const { authOptions } = require("../lib/nextAuth");
  let session = await unstable_getServerSession(
    context.req,
    context.res,
    authOptions
  );
  console.log("session", session);
  if ((session == null)) {
    context.res.statusCode = 403;
    return {
      props: {
        error: {
          kind: ERROR_UNAUTHORIZED,
          reason: REASON_NULL_SESSION,
          action: ACTION_ATTEMPT_AUTHENTICATION,
          message: MESSAGE_AUTHENTICATON_ATTEMPT_REQUIRED,
        },
      },
    };
  }

  // const prisma = require("../lib/prisma").default;
  const { Address, parse } = require("@universe/address-parser");
  console.log(context.req?.method);
  let situs = null;
  let uspsLabel = null;
  let text = null;
  let exactMatches = null;
  // destructure initial arguments from context?.query
  let queryAddress = context?.query?.address || null;
  let newAddress = null;
  if (queryAddress) {
    situs = parse(queryAddress);
    uspsLabel = Address.print(situs);
  }
  if (context.req.method === "GET" && queryAddress) {
    // search the database where the text is an exact match

    console.log(situs);
    console.log(uspsLabel);
    // try to find the address by parts
    // exactMatches = await prisma.address.findMany({
    //   where: {
    //     number: { equals: situs.number },
    //     streetName: { equals: situs.streetName },
    //     streetType: { equals: situs.streetType },
    //     city: { equals: situs.city },
    //     state: { equals: situs.state },
    //     country: { equals: situs.country },
    //   },
    // });
  }
  return {
    props: {
      address: {
        query: queryAddress,
        situs,
        uspsLabel,
        text,
        matches: { exact: exactMatches },
      },
    },
  };
}
