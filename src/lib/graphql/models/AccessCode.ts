import { Field, ID, ObjectType } from "type-graphql";
import { BaseEntity, Entity, Generated, OneToOne, PrimaryColumn } from "typeorm";
import { INode } from "./INode";

// Relay requires all 'objects' to have globally unique identifiers to allow data refetching
@ObjectType({implements: INode})
@Entity()
export class AccessCode extends INode {
    @Field()
    value:string


}