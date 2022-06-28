// access and authentication tools

export const UNAUTHORIZED_PRISMA_TRANSACTION = "UnauthorizedPrismaTransaction"
export const REQUIRED_SCOPE_MISSING = "REQUIRED_SCOPE_MISSING"

export class UnauthorizedDatabaseTransaction extends Error {
    reason:string
    scope:string | null
    constructor(message:string,reason:string,scope:string | null) {
        super(message);
        this.name = UNAUTHORIZED_PRISMA_TRANSACTION
        this.reason = reason
        this.scope = scope
    }
}

// used to help the client and server communicate errors
export const ERROR_UNAUTHORIZED = "UNAUTHORIZED";

// all scopes follow the format prefix:model:action
export const MODEL_SCOPE_PREFIX = "model"
export const ROLE_PREFIX= "role"
export const SCOPE_PREFIX="scope"
export const SCOPE_SEPERATOR = ":"
// scopes that allow permissions to be modified

// // allow an entity to 'see' scopes
// export const SCOPE_SCOPES_READ="scope:scopes:read"
//** allow an entity to assign scopes  */


export const SCOPE_SCOPES_GRANT=`scope:scopes:grant`
// allow an entity to revoke scopes
export const SCOPE_SCOPES_REVOKE="scope:scopes:revoke"
// // allow an entity to create a scope
// export const SCOPE_SCOPES_CREATE="scope:scopes:create"
// // allow an entity to delete a scope
// export const SCOPE_SCOPES_DELETE="scopes:scopes:delete"

// scopes that control access to gatekeeper

// allows navigation and general access to gatekeeper
export const SCOPE_GATEKEEPER_ACCESS = "scope:gatekeeper:access"
// allows read access to the address model
export const SCOPE_ADDRESS_READ = "scope:address:read"
// allows an entity to create an address in the system
export const SCOPE_ADDRESS_CREATE = "scope:address:create"
// allows an entity to update an address
export const SCOPE_ADDRESS_UPDATE = "scope:address:update"
// allows an entity to delete an address
export const SCOPE_ADDRESS_DELETE = "scope:address:delete"
// allows an entity to read access codes
export const SCOPE_ACCESS_CODE_READ = "scope:access_code:read"
// allows an entity to create access codes
export const SCOPE_ACCESS_CODE_CREATE = "scope:access_code:create"
// allows an entity to update an access code once created
export const SCOPE_ACCESS_CODE_UPDATE = "scope:access_code:update"
// allows an entity to delete an access code once created
export const SCOPE_ACCESS_CODE_DELETE = "scope:access_code:delete"