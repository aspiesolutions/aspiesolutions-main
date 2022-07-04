import "reflect-metadata"
import {Entity,Column,PrimaryGeneratedColumn,ManyToOne} from "typeorm"
import type {Relation} from "typeorm"
import { User } from "./User"
@Entity()
export class Session {
    @PrimaryGeneratedColumn({type:"bigint"})
    id:string
    @Column({type:"timestamptz",nullable:false})
    expires:Date
    @Column({type:"text",nullable:false})
    sessionToken: string
    @ManyToOne(type=>User, (user)=>user.id)
    user:Relation<User>
}