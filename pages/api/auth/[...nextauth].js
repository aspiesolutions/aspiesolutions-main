import NextAuth from "next-auth";

import { authOptions } from "../../../src/lib/nextAuth";

export default NextAuth(authOptions);
