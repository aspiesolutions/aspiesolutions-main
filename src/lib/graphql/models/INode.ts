import { Field, ID, ObjectType } from "type-graphql";
import { BaseEntity, Column, Entity, Generated, PrimaryColumn } from "typeorm";
// node acts as a mapping class that associates a public UUID with an internal ID to allow refetching

export abstract class INode extends BaseEntity {
    @Field(type=>ID)
    @PrimaryColumn({type:"uuid"})
    @Generated("uuid")
    id: string
}