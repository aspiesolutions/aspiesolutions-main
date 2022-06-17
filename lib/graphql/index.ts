import "reflect-metadata"
import { Field, ID, ObjectType } from "type-graphql";
@ObjectType()
class GQLState {
    @Field(type=>ID)
    id: string;
    @Field()
    abbv: string;
    @Field()
    full_text: string
    @Field(type=>[GQLCity]!)
    cities: [GQLCity]
    @Field()
    addresses: GQLAddress
}
@ObjectType()
class GQLCity {
    @Field(type=>ID)
    id: string;
    @Field()
    text: string;
    @Field(type=>[GQLAddress])
    addresses:[GQLAddress]
}
@ObjectType()
class GQLAddress {
    @Field(type=> ID)
    id: string;
    @Field()
    street: string;
    @Field({nullable:true})
    buisness_name?: string
    @Field()
    city: GQLCity
    @Field()
    state: GQLState
    @Field()
    zip: string
    @Field()
    access_codes: [GQLAccessCode]

}
@ObjectType()
class GQLAccessCode {
    @Field(type=>ID)
    id: string;
    @Field()
    used_for:string
    @Field()
    value:string
}