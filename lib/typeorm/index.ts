import { AppDataSource } from "./data-source"
import { User } from "./entity/User"
import { AccessCode } from "./entity/AccessCode"

export const datasource = AppDataSource.initialize()
