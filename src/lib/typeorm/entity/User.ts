import { Entity, PrimaryGeneratedColumn, Column, OneToMany } from "typeorm"
import { Account } from "./Account"
import type {Relation} from "typeorm"
import { Session } from "./Session"
import { AdapterUser } from "next-auth/adapters"
@Entity()
export class User {
    @PrimaryGeneratedColumn({type:"integer"})
    id: string
    @Column({type:"text",nullable:true})
    name: string | null
    @Column({type:"text",unique:true,nullable:true})
    email: string | null
    @Column({type:"timestamptz",nullable:true})
    emailVerified:Date | null
    @Column({type:"text",nullable:true})
    image:string
    @OneToMany(()=>Account,(account)=>account.id)
    accounts:Relation<Account[]>
    @OneToMany(()=>Session,(session)=>session.id)
    sessions:Relation<Session[]>

}
