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
    @Column({nullable:true})
    refresh_token?:string
    @Column({nullable:false})
    access_token:string
    @Column({nullable:false})
    expires_at:number
    @Column({nullable:false})
    token_type:string
    @Column({type:"text"})
    scope:string
    @Column({nullable:true})
    session_state?:string
    @Column({nullable:true})
    oauth_token_secret?: string
    @Column({nullable:true})
    oauth_token?:string

}