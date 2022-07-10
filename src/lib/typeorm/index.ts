import { AppDataSource } from "../data-source"
import { User } from "../graphql/models/User"
import { AccessCode } from "../graphql/models/AccessCode"
import { Session } from "../graphql/models/Session"
import { Account } from "../graphql/models/Account"

export const datasource = AppDataSource.initialize()
