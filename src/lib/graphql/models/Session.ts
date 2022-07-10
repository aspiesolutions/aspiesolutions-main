import "reflect-metadata"
import {Entity,Column,PrimaryColumn,ManyToOne, Generated} from "typeorm"
import type {Relation} from "typeorm"
import { User } from "./User"
@Entity()
// Relay requires all 'objects' to have globally unique identifiers to allow data refetching
export class Session {
    @PrimaryColumn({type:"uuid",nullable:false,unique:true})
    @Generated("uuid")
    id:string
    @Column({type:"timestamptz",nullable:false})
    expires:Date
    @Column({type:"text",nullable:false})
    sessionToken: string
    @ManyToOne(type=>User, (user)=>user.id)
    user:Relation<User>
}