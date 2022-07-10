import { Entity, PrimaryColumn, Column, OneToMany, Generated } from "typeorm"
import { Account } from "./Account"
import type {Relation} from "typeorm"
import { Session } from "./Session"
import { AdapterUser } from "next-auth/adapters"
// Relay requires all 'objects' to have globally unique identifiers to allow data refetching
@Entity()
export class User {
    @PrimaryColumn({type:"uuid",nullable:false,unique:true})
    @Generated("uuid")
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
