const _ = require("lodash")
const path = require("next/dist/shared/lib/isomorphic/path")

// const withOffline = require("next-offline")
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental:{
  },
  webpack: (defaultConfig,{isServer,webpack})=>{
    let config = {
      experiments: {
        asyncWebAssembly:true,
        topLevelAwait: true
      },
      node: {
        __dirname:false,
      },
      module:defaultConfig.module || {
        rules: defaultConfig.module.rules || []
      },
      externals: defaultConfig.externals || ["@aspiesolutions/neon-lib-db-sea-orm/index.node"],
      plugins:defaultConfig.plugins || [],
      resolve: {
        alias: {
          typeorm:path.resolve(__dirname,"./node_modules/typeorm/typeorm-model-shim")
        }
      }
    }
    if(!isServer) {
      config.plugins.push(new webpack.IgnorePlugin({resourceRegExp:/src\/lib\/server/}))
      // config.plugins.push(new webpack.IgnorePlugin({resourceRegExp:/\.node$/}))
      config.module.rules.push({
        test:/\.node$/,
        loader:'node-loader'
      })
    }

    return _.merge(defaultConfig,config)
  }
}

module.exports = nextConfig
