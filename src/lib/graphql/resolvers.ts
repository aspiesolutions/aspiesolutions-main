import "reflect-metadata"
import {Resolver,Query} from "type-graphql"
import { AccessCode } from "./models/AccessCode"
import { Address } from "./models/Address"
@Resolver()
export class AccessCodeResolver {
    @Query(returns => AccessCode)
    async accessCode() {
        return "12345"
    }
}
