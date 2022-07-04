import "reflect-metadata"
import {Resolver,Query} from "type-graphql"
@Resolver()
export class AccessCodeResolver {
    @Query(returns => String)
    async accessCode() {
        return "12345"
    }
}