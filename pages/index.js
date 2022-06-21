import Head from "next/head";
// import Image from 'next/image'
import mapboxgl from "!mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import "@mapbox/mapbox-gl-geocoder/dist/mapbox-gl-geocoder.css";
import MapboxGeocoder from "@mapbox/mapbox-gl-geocoder";
// import { Address } from '@universe/address-parser'
import { useCallback, useEffect, useRef, useState } from "react";
import styles from "../styles/Home.module.css";
import { gql, useMutation, useQuery } from "urql";
import { useRouter } from "next/router";
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

export default function Home(props) {
  // check for geolocation
  const router = useRouter();
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
    if (map.current) return;
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
    if (!map.current) return;
    map.current.on("move", () => {
      setLng(map.current.getCenter().lng.toFixed(16));
      setLat(map.current.getCenter().lat.toFixed(16));
      setZoom(map.current.getZoom());
    });
  });
  useEffect(() => {
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
  }, [router, mapboxAddressResult]);
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
  // cities and states can be fetched on the server side first
  const prisma = require("../lib/prisma").default;
  const { Address, parse } = require("@universe/address-parser");
  let situs = null;
  let uspsLabel = null;
  let text = null;
  let exactMatches = null;
  let queryAddress = context?.query?.address || null;
  let newAddress = null
  if (queryAddress) {
    // search the database where the text is an exact match
    situs = parse(queryAddress);
    uspsLabel = Address.print(situs);
    console.log(situs);
    console.log(uspsLabel);
    // try to find the address by parts
    exactMatches = await prisma.address.findMany({
      where: {
        number: { equals: situs.number },
        streetName: { equals: situs.streetName },
        streetType: { equals: situs.streetType },
        city: { equals: situs.city },
        state: { equals: situs.state },
        country: { equals: situs.country },
      },
    });
  }
  if(exactMatches.length == 0) {
    try {

      newAddress = await prisma.address.create({
        data:{...situs,
          text:uspsLabel}
        })
        console.log(exactMatches)
      }
      catch(newAddressError) {
        console.error(newAddressError)
      }
  }
  console.log(exactMatches);
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
