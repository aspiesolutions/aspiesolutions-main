import "reflect-metadata"
import {Entity,Column,ManyToOne, PrimaryColumn, Generated} from "typeorm"
import type {Relation} from "typeorm"
import { User } from "./User"
import { Field, ID, ObjectType } from "type-graphql"
//We are using babel, types must be explicitly declared
// in the decorator for the ORM to work correctly
@ObjectType()
@Entity()
export class Account {
    @Field(type=>ID)
    @PrimaryColumn({type:"uuid",nullable:false,unique:true})
    @Generated("uuid")
    id:string
    @ManyToOne(()=>User,(user)=>user.id)
    user:Relation<User>
    @Column({type:"text",nullable:false})
    type:string
    @Column({type:"text",nullable:false})
    provider:string
    @Column({type:"text",nullable:false})
    providerAccountId:string
    @Column({type:"text",nullable:true})
    refresh_token:string | null
    @Column({type:"text",nullable:false})
    access_token:string
    @Column({type:"int",nullable:false})
    expires_at:number
    @Column({type:"text",nullable:true})
    id_token:string
    @Column({type:"text",nullable:false})
    token_type:string
    @Column({type:"text"})
    scope:string
    @Column({type:"text",nullable:true})
    session_state:string | null
    @Column({type:"text",nullable:true})
    oauth_token_secret: string | null
    @Column({type:"text",nullable:true})
    oauth_token:string | null

}