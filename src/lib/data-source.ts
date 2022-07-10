import "reflect-metadata"
import { DataSource } from "typeorm"
import { Account } from "./graphql/models/Account"
import { Session } from "./graphql/models/Session"
import { User } from "./graphql/models/User"
import { AccessCode } from "./graphql/models/AccessCode"
import { VerificationToken } from "./graphql/models/VerificationToken"
import { Node } from "./graphql/models/INode"
import { Address } from "@universe/address-parser"


const PG_USERNAME = process.env.PG_USERNAME || "web"
const PG_PASSWORD =  process.env.PG_PASSWORD || "test"
const PG_DATABASE = process.env.PG_DATABASE || "test"
const DB_IS_PRODUCTION = process.env.NODE_ENV === "production"
const PG_PORT =  parseInt(process.env.PG_PORT) || 5432
const PG_HOST = process.env.PG_HOST || "localhost"
// this is a public certificate placed here for simplicity sake
const CA_CERTIFICATE = process.env.CA_CERFITICATE ||
`-----BEGIN CERTIFICATE-----
MIIEMTCCApmgAwIBAgIEAJGIBzANBgkqhkiG9w0BAQwFADA6MTgwNgYDVQQDDC9j
ZmE5NDdlNC1kZjQ0LTQ3ODUtOTBkOC1jYjk0YWNhMWU0OWYgUHJvamVjdCBDQTAe
Fw0yMDAzMjEwMTU0MzVaFw0zMDAzMTkwMTU0MzVaMDoxODA2BgNVBAMML2NmYTk0
N2U0LWRmNDQtNDc4NS05MGQ4LWNiOTRhY2ExZTQ5ZiBQcm9qZWN0IENBMIIBojAN
BgkqhkiG9w0BAQEFAAOCAY8AMIIBigKCAYEA2DM2cRLZCvssymdAOwiVpW8T2sBa
QewRblh7icK5KhsKm6fSZ3Qi0dx28sxezVEWMFEwZBuZH9JEShbnkSAzaa1CGzCb
0ZtBC0eat2QOYEKeuUYk1uavDyr+RMiuNkUTQNz7atnFHpreqHWzBdlAlNPgLkWv
a7Wleh2fOkioFU2IXdQcJl8/fu1Ya8TefvKxFpSV2vm0wjtPsMyL1RLXvdI1TUcv
MIuTDFMNUXpy/6lhJZ7CwOTxysmDiqi61BnsHUt7sQd4mVE5QGxvlcJaIDXP1hbR
e4zS+xWIW73jIQIFTxvqyzlRjTKWzz+Gsxi2U+ofXHJPWyxweOiEYwHKsfPpkvGG
Y1XWV8RH/1/N+yTenTPEEXeVS37miVv07eTs+lFMEYIDHTkms9fxdJyhbu9JJPoD
sxdpEaAO2tPwxpnKkBIiCuHSwgSF1KOeJXzGfynsiZJY72gxzYCpgTt1zX3OKcQy
tqxvzcCPyVaoV2A2n2vbvhSmGDvsIoz+VZCFAgMBAAGjPzA9MB0GA1UdDgQWBBRo
Bku8nMslL2Jel7vFiavqNDF/eTAPBgNVHRMECDAGAQH/AgEAMAsGA1UdDwQEAwIB
BjANBgkqhkiG9w0BAQwFAAOCAYEAqOj5OCk3C171gSYKE4viOPL06P+x0yzjFgTw
O1wzFcHU1agr5uKe9DvN39GMPP1wU4EZy4pjBzouF0YGbNkX6zIjwgoLnwdwUscC
j6OtC1DC3TG8QqjnAz0+Tep7YrHSjLYt8/J0+//HBqkU+L94lEfBUKAdNVwdQMUA
HASUXkOUVu6doM+mJjW12R/7+S9D6+kVO5cuOFRF67w8BJjDHHK4YnslfDIrYSxF
pEMuB2oLi0kgxFwWVmm63mll69IXFBaN6OwuH21cN8P/MGiwjXXHF1f65eU+JbIa
7yCT4NQUPcuxr1fa4ioYOlZRsWZ/Qt6eCl6dbWxfszm9bO/VPlSyEjsKWURBjZqJ
JfNaN9M9k+jOCz+F6K4AkmFuSwa5+seMnITjuUOeCkXZ5qEqd8U/M+Hh0Koy+Kmu
OtdqChDDveYbOMYBxFswj88edWy49m/n1DviVPlCsvJYt8PaBiWmCWWfKbsUOFtY
OmXSaq7DUuq4eeXJUfpdUGnQx578
-----END CERTIFICATE-----`


export const AppDataSource = new DataSource({
    type: "postgres",
    host: PG_HOST,
    port:PG_PORT,
    username: PG_USERNAME,
    password: PG_PASSWORD,
    database: PG_DATABASE,
    ssl: {ca:CA_CERTIFICATE},
    synchronize: !DB_IS_PRODUCTION,
    logging: false,
    entities: [User,Account, VerificationToken,Session,AccessCode, Address],
    migrations: [],
    subscribers: [],
})

// export const PG_CONNECTION_STRING = `postgres://${PG_USERNAME}:${PG_PASSWORD}@${PG_HOST}:${PG_PORT}/${PG_DATABASE}?sslmode=require`