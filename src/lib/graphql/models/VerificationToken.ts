import {Entity, Column, PrimaryColumn, Generated} from "typeorm"
@Entity()
// Relay requires all 'objects' to have globally unique identifiers to allow data refetching
export class VerificationToken {
 @PrimaryColumn({type:"uuid"})
 @Generated("uuid")
 id: string
 @Column({type:"text", nullable:false})
 token:string
 @Column({type:"timestamptz",nullable:false})
 expires:Date
}