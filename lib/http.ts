import log from "../lib/logger"

export enum AUTH_TYPE {
    Basic = "Basic",
    Digest = "Digest",
    Bearer = "Bearer"
}
export function parseAuthenticationHeader(header:string | null | undefined): null {
    if(typeof header === "undefined" || header == null) {
        return null;
    }

    log.info("parseAuthenticationHeader",header)
    return null
}