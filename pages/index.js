import Head from 'next/head'
import Image from 'next/image'
import { Address,parse } from '@universe/address-parser'
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
export default function Home() {
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
        <form method='POST' onSubmit={async (e)=>{
          e.preventDefault()
          let addrInput = e.currentTarget.querySelector(`input[name="address"]`);
          if(addrInput == null) {
            return false
          }
          let address = await parse(addrInput.value)
          console.log("address")

          return false
        }}>
          <label htmlFor="address">address</label>
          <input id="address" type="text" name="address" placeholder='123 main street, example city, 12345' />
          <button type="submit">Search</button>
        </form>
      </main>
    </div>
  )
}


export async function getServerSideProps(context) {
  // cities and states can be fetched on the server side first
  const prisma = require("../lib/prisma").default

  return {props:{states:await prisma.state.findMany()}}
}