import "reflect-metadata"
import {Entity,PrimaryGeneratedColumn,Column,ManyToOne} from "typeorm"
import type {Relation} from "typeorm"
import { User } from "./User"
@Entity()
export class Account {
    @PrimaryGeneratedColumn()
    id:string
    @ManyToOne(()=>User,(user)=>user.id)
    user:Relation<User>
    @Column()
    type:string
    @Column()
    provider:string
    @Column()
    providerAccountId:string
    @Column({type:"text",nullable:true})
    refresh_token:string | null
    @Column({nullable:false})
    access_token:string
    @Column({nullable:false})
    expires_at:number
    @Column({nullable:true})
    id_token:string
    @Column({nullable:false})
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