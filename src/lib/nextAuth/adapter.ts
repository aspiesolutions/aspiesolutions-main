import { DataSource } from "typeorm";
import { User } from "../typeorm/entity/User";
import log from "../logger";
import { Account } from "../typeorm/entity/Account";
import type { FindOptionsWhere } from "typeorm";
import { Session } from "../typeorm/entity/Session";

export type Options = { datasource: DataSource };

let NOT_IMPLEMENTED_MSG = "Adapter Method Not Implemented";
import type { Adapter, AdapterUser } from "next-auth/adapters";
// import { Awaitable } from "next-auth";
export default function CustomTypeOrmAapter(options: Options): Adapter {
  console.log("initialize adapter");
  let datasourcePromise = options.datasource.initialize();
  async function getUserRepository() {
    return await (await datasourcePromise).getRepository(User);
  }
  async function getAccountRepository() {
    return await (await datasourcePromise).getRepository(Account);
  }
  async function getSessionRepository() {
    return await (await datasourcePromise).getRepository(Session);
  }
  return {
    async createUser(user): Promise<AdapterUser> {
      let userRepository = await getUserRepository();
      let entity = userRepository.create(user);
      let userCreateResult = await userRepository.save(entity);
      // throw new Error(NOT_IMPLEMENTED_MSG)
      let adapterUser: AdapterUser = { ...userCreateResult };
      return adapterUser;
    },
    async getUser(id) {
      let userRepository = await getUserRepository();
      console.log("adapter getUser", id);
      let findOptions: FindOptionsWhere<User> = {
        id,
      };
      let getUserResult = await userRepository.findOneBy(findOptions);
      log.debug("getUserResult isNull", getUserResult === null);
      return { ...getUserResult } as AdapterUser;
    },
    async getUserByEmail(email) {
      console.debug("getUserByEmail");
      // throw new Error(NOT_IMPLEMENTED_MSG)
      let userRepository = await getUserRepository();
      let findOneByOptions: FindOptionsWhere<User> = {
        email,
      };
      let user = await userRepository.findOneBy(findOneByOptions);
      return user ? ({ ...user } as AdapterUser) : null;
    },
    async getUserByAccount({ providerAccountId, provider }) {
      console.debug("getUserByAccount", providerAccountId, provider);
      let accountRepository = await getAccountRepository();
      let findOptions: FindOptionsWhere<Account> = {
        provider,
        providerAccountId,
      };
      let account = await accountRepository.findOne({
        where: findOptions,
        relations: ["user"],
      });
      console.log("account", account);
      if (account === null) {
        log.debug("did not find an account");
        return null;
      }
      if (account?.user == null) {
        log.debug("This account is not linked to any user");
        return null;
      }
      log.debug("Found an account with an associated user");

      return { ...account.user };
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
        return;
      }
      let accountRepository = await getAccountRepository();
      // find the account first before creating it.
      let existingAccount = await accountRepository.findOneBy({
        provider: account.provider,
        providerAccountId: account.providerAccountId,
      });
      if (existingAccount == null) {
        // if the account does not already exist, create it, and link it to the user
        log.debug("account does not exist. creating it");
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

        newAccount.oauth_token =
          typeof account?.oauth_token === "string" ? account.oauth_token : null;
        newAccount.oauth_token_secret =
          typeof account?.oauth_token_secret === "string"
            ? account.oauth_token_secret
            : null;
        newAccount.id_token =
          typeof account?.id_token === "string" ? account.id_token : null;
        existingAccount = await accountRepository.save(newAccount);
        // update the accounts with the user
      }
      // associate this account with the current user
      existingAccount.user = user;
      // overwrite the value with the values from the database
      existingAccount = await accountRepository.save(account);
      // this function appears to be expected to return nothing
      return;
      // return {
      //   access_token: existingAccount.access_token,
      //   token_type: existingAccount.token_type,
      //   id_token: existingAccount.id_token,
      //   refresh_token: existingAccount.refresh_token,
      //   scope:existingAccount.scope,
      //   expires_at: existingAccount.expires_at,
      //   session_state: existingAccount.session_state
      // };
      // return {}
    },
    async unlinkAccount({ providerAccountId, provider }) {
      console.debug("UnlinkAccount");
      // throw new Error(NOT_IMPLEMENTED_MSG)

      return null;
    },
    async createSession({ sessionToken, userId, expires }) {
      let currentTime = Date.now();
      if (currentTime > expires.getTime()) {
        log.debug("session expired before it was created!");
        return null;
      }
      let userRepository = await getUserRepository();
      let user = await userRepository.findOneBy({
        id: userId,
      });
      // get the user for this session
      if (user === null) {
        log.debug("Tried create/associate a session for a non existant user!");
        return null;
      }
      // create the session for this user
      let sessionRepository = await getSessionRepository();
      let newSession = sessionRepository.create({
        sessionToken,
        user,
        expires,
      });
      newSession.user = user;
      let savedSession = await sessionRepository.save(newSession);
      return {
        id: savedSession.id,
        sessionToken: savedSession.sessionToken,
        userId: savedSession.user.id,
        expires: savedSession.expires,
      };
    },
    async getSessionAndUser(sessionToken) {
      let currentTime = Date.now();
      let sessionRepository = await getSessionRepository();

      let session = await sessionRepository.findOne({
        where: { sessionToken },
        relations: ["user"],
      });

      if (session === null || session.user == null) {
        return null;
      }
      // reshape the return object to the expected values
      let user = session.user;
      return {
        session: {
          id: session.id,
          sessionToken: session.sessionToken,
          userId: user.id,
          expires: session.expires,
        },
        user: {
          id: user.id,
          emailVerified: user.emailVerified,
          name: user.name,
          email: user.email,
          image: user.image,
        },
      };
    },
    async updateSession({ sessionToken, userId, expires }) {
      // throw new Error(NOT_IMPLEMENTED_MSG)
      let sessionRepository = await getSessionRepository();
      let session = await sessionRepository.findOneBy({
        sessionToken,
        user: { id: userId },
      });
      if (session === null) {
        return null;
      } else {
        session.expires = expires;
        return { ...(await sessionRepository.save(session)), userId };
      }
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
