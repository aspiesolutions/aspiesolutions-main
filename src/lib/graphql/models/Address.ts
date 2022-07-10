import { ObjectType } from "type-graphql";
import { Entity, OneToOne, PrimaryColumn } from "typeorm";
@ObjectType()
@Entity()
export class Address {
    @PrimaryColumn({type:"uuid"})
    id:string
    // @OneToOne(type=>Node)
    // node:Node
}