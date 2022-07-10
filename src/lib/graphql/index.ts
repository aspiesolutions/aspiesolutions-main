import "reflect-metadata"
import {buildSchema} from "type-graphql"
import {AccessCodeResolver} from "./resolvers"

const build = async ()=>{
    return await buildSchema({resolvers:[AccessCodeResolver],})
}

export default build