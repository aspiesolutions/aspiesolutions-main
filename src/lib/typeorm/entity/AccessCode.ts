import { Entity, Generated, PrimaryColumn } from "typeorm";

@Entity()
// Relay requires all 'objects' to have globally unique identifiers to allow data refetching
export class AccessCode {
    @PrimaryColumn({type:"uuid",nullable:false,unique:true})
    @Generated("uuid")
    id:string
}