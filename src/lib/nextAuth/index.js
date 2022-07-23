import Auth0Provider from "next-auth/providers/auth0";
// import { PrismaAdapter } from "@next-auth/prisma-adapter";
// import prisma from "./prisma";
import { NEXTAUTH_AUTH0_PROVIDER_ID } from "../constants";
import CustomTypeOrmAdapter from "./adapter"
// import blank from "next-auth/adapters/"
import log from "../logger"
// import { AppDataSource, PG_CONNECTION_STRING } from "../data-source";
export const authOptions ={
    // adapter: CustomTypeOrmAdapter({datasource:AppDataSource}),
    logger:{
      error(code,metadata) {
        log.error(code,metadata)
        console.dir(metadata)
      },
      warn(code) {
        log.warn(code)
      },
      debug(code,metadata) {
        log.debug(code,metadata)
      }
    },
    callbacks:{
      async session({session,token,user}) {
        // we are using databasae sessions, find the account in the database, and paste the access token into the session
        // the api expects an access token in the session
        // let account = await prisma.account.findFirst({where:{userId:{equals:user.id}}})
        // console.log(account)
        // if(account) {
        //   session.account = session.account || {};
        //   session.account.accessToken = account.access_token
        //   session.account.scope = account.scope
        // }
        return session

      },
      async jwt(token,user) {
        console.log("token callback",token,user)
        if (user) {
          token = { accessToken: user.accessToken }
      }
  
      return token
      }
    },
    providers: [
      Auth0Provider({
        id:NEXTAUTH_AUTH0_PROVIDER_ID,   
        clientId: process.env.AUTH0_CLIENT_ID,
        clientSecret: process.env.AUTH0_CLIENT_SECRET,
        issuer: process.env.AUTH0_ISSUER,
      }),
    ],
  }