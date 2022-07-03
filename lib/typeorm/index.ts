import { AppDataSource } from "./data-source"
import { User } from "./entity/User"
import { AccessCode } from "./entity/AccessCode"
import { Session } from "./entity/Session"
import { Account } from "./entity/Account"

export const datasource = AppDataSource.initialize()
