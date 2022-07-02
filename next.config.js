const _ = require("lodash")
const path = require("next/dist/shared/lib/isomorphic/path")
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
      },
      resolve: {
        alias: {
          typeorm:path.resolve(__dirname,"./node_modules/typeorm/typeorm-model-shim")
        }
      }
    }
    return _.merge(defaultConfig,config)
  }
}

module.exports = nextConfig
