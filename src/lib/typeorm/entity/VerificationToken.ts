import {Entity, PrimaryGeneratedColumn,Column} from "typeorm"
@Entity()
export class VerificationToken {
 @PrimaryGeneratedColumn({type:"bigint"})
 id: number
 @Column({type:"text", nullable:false})
 token:string
 @Column({type:"timestamptz",nullable:false})
 expires:Date
}