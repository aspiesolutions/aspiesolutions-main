import Auth0Provider from "next-auth/providers/auth0";
import { PrismaAdapter } from "@next-auth/prisma-adapter";
import prisma from "./prisma";
import { NEXTAUTH_AUTH0_PROVIDER_ID } from "./constants";


export const authOptions ={
    adapter: PrismaAdapter(prisma),
    providers: [
      Auth0Provider({
        id:NEXTAUTH_AUTH0_PROVIDER_ID,   
        clientId: process.env.AUTH0_CLIENT_ID,
        clientSecret: process.env.AUTH0_CLIENT_SECRET,
        issuer: process.env.AUTH0_ISSUER,
      }),
    ],
  }