import { Entity, PrimaryGeneratedColumn, Column, OneToMany } from "typeorm"
import { Account } from "./Account"
import type {Relation} from "typeorm"
import { Session } from "./Session"
@Entity()
export class User {
    @PrimaryGeneratedColumn({type:"integer"})
    id: string
    @Column()
    name: string
    @Column({unique:true,nullable:true})
    email?: string
    @Column({type:"timestamptz",nullable:true})
    emailVerified?:Date
    @Column({type:"text",nullable:true})
    image:string
    @OneToMany(()=>Account,(account)=>account.id)
    accounts:Relation<Account[]>
    @OneToMany(()=>Session,(session)=>session.id)
    sessions:Relation<Session[]>

}
