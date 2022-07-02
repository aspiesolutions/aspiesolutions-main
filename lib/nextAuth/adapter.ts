import { DataSource } from "typeorm";
import { User } from "../typeorm/entity/User";
import log from "../logger";
import { Account } from "../typeorm/entity/Account";
import type { FindOptionsWhere } from "typeorm";
import { Session } from "../typeorm/entity/Session";

export type Options = { datasource: DataSource };

let NOT_IMPLEMENTED_MSG = "Adapter Method Not Implemented";

/** @return { import("next-auth/adapters").Adapter } */
export default function CustomTypeOrmAapter(options: Options) {
  console.log("initialize adapter");
  let datasourcePromise = options.datasource.initialize();
  async function getUserRepository() {
    return await (await datasourcePromise).getRepository(User);
  }
  async function getAccountRepository() {
    return await (await datasourcePromise).getRepository(Account);
  }
  async function getSessionRepository() {
    return await (await datasourcePromise).getRepository(Session)
  }
  return {
    async createUser(user) {
      let userRepository = await getUserRepository();
      let entity = userRepository.create(user);
      let userCreateResult = await userRepository.save(entity);
      console.log("createUserResult", userCreateResult);
      // throw new Error(NOT_IMPLEMENTED_MSG)
      return userCreateResult;
    },
    async getUser(id) {
      let userRepository = await getUserRepository();
      console.log("adapter getUser", id);
      let findOptions: FindOptionsWhere<User> = {
        id,
      };
      let getUserResult = await userRepository.findOneBy(findOptions);
      log.debug("getUserResult isNull", getUserResult === null);
      return getUserResult;
    },
    async getUserByEmail(email) {
      console.debug("getUserByEmail");
      // throw new Error(NOT_IMPLEMENTED_MSG)
      let userRepository = await getUserRepository();
      let findOneByOptions: FindOptionsWhere<User> = {
        email,
      };
      let getuserByEmailResult = await userRepository.findOneBy(
        findOneByOptions
      );
      console.debug(
        "getUserByEmailResult isNull",
        getuserByEmailResult === null
      );
      console.debug(getuserByEmailResult);
      return getuserByEmailResult;
    },
    async getUserByAccount({ providerAccountId, provider }) {
      console.debug("getUserByAccount", providerAccountId, provider);
      let accountRepository = await getAccountRepository();
      let findOptions: FindOptionsWhere<Account> = {
        provider,
        providerAccountId,
      };
      let account = await accountRepository.findOne({where:findOptions, relations:["user"]});
      console.log("account",account)
      if (account === null) {
        console.log("did not find an account")
        log.debug("did not find an account");
        return null;
      }
      if (account?.user == null) {
        log.debug("This account is not linked to any user");
        return null;
      }
      log.debug("Found an account with an associated user");

      return account?.user;
    },
    async updateUser(user) {
      console.debug("update user");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async deleteUser(userId) {
      console.debug("deleteUser");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async linkAccount(account) {
      console.debug("linkAccount", account);
      // get the user for this account
      let userRepository = await getUserRepository();
      let user = await userRepository.findOneBy({ id: account.userId });
      if (user === null) {
        // dont continue if the user is not found
        throw new Error(`the user with userId ${account?.id} was not found. `);
      }
      let accountRepository = await getAccountRepository();
      // find the account first before creating it.
      let existingAccount = await accountRepository.findOneBy({
        provider: account.provider,
        providerAccountId: account.providerAccountId,
      });
      if (existingAccount == null) {
        // if the account does not already exist, create it, and link it to the user
        log.debug("account does not exist. creating it")
        let newAccount = new Account();
        newAccount.user = user;
        newAccount.type = account.type;
        newAccount.provider = account.provider;
        newAccount.providerAccountId = account.providerAccountId;
        newAccount.refresh_token = account?.refresh_token;
        newAccount.access_token = account.access_token;
        newAccount.expires_at = account?.expires_at;
        newAccount.token_type = account?.token_type;
        newAccount.scope = account.scope;
        newAccount.session_state = account.session_state;
        newAccount.oauth_token = account?.oauth_token;
        newAccount.oauth_token_secret = account?.oauth;
        return await accountRepository.save(newAccount);
        // update the accounts with the user
      }
      existingAccount.user = user;
      return await accountRepository.save(account);
    },
    async unlinkAccount({ providerAccountId, provider }) {
      console.debug("UnlinkAccount");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async createSession({ sessionToken, userId, expires }) {
        let currentTime = Date.now();
        if(currentTime > expires) {
            log.debug("session expired before it was created!")
            return null
        }
        let userRepository = await getUserRepository()
        let user = await userRepository.findOneBy({
            id:userId
        })
        // get the user for this session
        if(user === null) {
            log.debug("Tried create/associate a session for a non existant user!")
            return null;
        }
        // create the session for this user
        let sessionRepository = await getSessionRepository();
        let newSession = sessionRepository.create({sessionToken,user,expires})
        newSession.user = user
        return await sessionRepository.save(newSession)
    },
    async getSessionAndUser(sessionToken) {
      let userRepository = await getUserRepository()
      let sessionRepository = await getSessionRepository();

      let session = await sessionRepository.findOne({where:{sessionToken},relations:["user"]})
      if( session === null) {
        return null;
      }
      if(session.user == null) {
        return null;
      }
      console.log("getSessionAndUser session.user",session.user)

      return session.user
    },
    async updateSession({ sessionToken }) {
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async deleteSession(sessionToken) {
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async createVerificationToken({ identifier, expires, token }) {
      console.log("createVerificationToken");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async useVerificationToken({ identifier, token }) {
      console.log("useVerificationToken");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
  };
}
