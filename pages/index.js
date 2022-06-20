import Head from 'next/head'
import Image from 'next/image'
import mapboxgl from "!mapbox-gl"
// import { Address } from '@universe/address-parser'
import { useCallback, useEffect, useRef, useState } from 'react'
import styles from '../styles/Home.module.css'
// import parseAddress from "../lib/parseAddress"
const defaultMapboxToken = "pk.eyJ1IjoianRoZWN5YmVydGlua2VyZXIiLCJhIjoiY2w0bjRicWFzMWs2eTNpcGd5c2UyYm1tbCJ9.gtMxHjwKheor-JFsyfx19g"
mapboxgl.accessToken = defaultMapboxToken
function checkGeoLocation(){
  console.log("checking for geolocation")
  if(typeof window === "object" && "navigator" in window) {
    console.log("window.navigator is present")
    try {
      window.navigator.geolocation.getCurrentPosition(console.log,setGeoError)
    }
    catch (geoError) {
      console.error("geolocation is not available")
    }
  }
}
export default function Home(props) {
  // check for geolocation
  const mapboxContainer = useRef(null);
  const map = useRef(null);
  const [lat,setLat] = useState(0)
  const [lng,setLng] = useState(0)
  const [zoom, setZoom] = useState(1)
  useEffect(()=>{
    if(map.current) return;
    map.current = new mapboxgl.Map({
      container:mapboxContainer?.current,
      style:"mapbox://styles/mapbox/streets-v11",
      center:[50,50],
      zoom,

    })
    map.current.addControl(
      new mapboxgl.GeolocateControl({
        positionOptions: {
          enableHighAccuracy:true
        },
        trackUserLocation:true,
        showUserHeading:true
      })
    )
  })
  useEffect(()=>{
    if(!map.current) return;
    map.current.on('move',()=>{
      setLng(map.current.getCenter().lng.toFixed(16))
      setLat(map.current.getCenter().lat.toFixed(16))
      setZoom(map.current.getZoom())
    })
  })
  return (
    <div className={styles.container}>
      <Head>
        <title>Gatekeeper</title>
        <meta name="description" content="Track Access Codes" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <form method='GET'>
          <label htmlFor="address">address</label>
          <input id="address" type="text" name="address" defaultValue={props?.address?.query} placeholder='123 main street, example city, 12345' />
          <button type="submit">Search</button>
        </form>
      </main>
      <div>{zoom} {lat}, {lng}</div>
      <div style={{minWidth:"1px"}} ref={mapboxContainer}></div>
    </div>
  )
}


export async function getServerSideProps(context) {
  // cities and states can be fetched on the server side first
  const prisma = require("../lib/prisma").default
  let situs = null;
  let uspsLabel = null;
  let text = null;
  let queryAddress = context?.query?.address || null;
  if(queryAddress) {
    const {Address, parse} = require("@universe/address-parser");
    console.log("address",queryAddress)
    try {
      situs = await parse(queryAddress)
      let address = new Address(situs)
      let uspsLabel = address.label()
      let text = address.print()
    }
    catch (parseError) {
      console.error(parseError)
    }
    console.log(situs)
  }

  return {props:{address:{query:queryAddress,situs,uspsLabel,text}}}
}