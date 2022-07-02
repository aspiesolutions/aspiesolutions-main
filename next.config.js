const _ = require("lodash")
// const withOffline = require("next-offline")
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental:{
  },
  webpack: (defaultConfig)=>{
    let config = {
      experiments: {
        asyncWebAssembly:true,
        topLevelAwait: true
      }
    }
    return _.merge(defaultConfig,config)
  }
}

module.exports = nextConfig
