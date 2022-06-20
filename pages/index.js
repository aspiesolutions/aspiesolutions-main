import Head from 'next/head'
import Image from 'next/image'
// import { Address } from '@universe/address-parser'
import { useCallback, useEffect, useState } from 'react'
import styles from '../styles/Home.module.css'
// import parseAddress from "../lib/parseAddress"
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
  const [geoError,setGeoError]= useState(null)

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