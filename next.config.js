/** @type {import('next').NextConfig} */
const _ = require("lodash")
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
