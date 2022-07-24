import TransactionOverView from "../../components/TransactionOverView"
import { Transaction } from "../../src/lib/models/Transaction"
describe('TestRelay.cy.ts', () => {
  let dummyTransaction: Transaction = {
    id:"dummy-id",
    senderId:"dummy-sender-id",
    recieverId:"dummy-reciever-id",
    originatorId: "dummy-sender-id",
    amount: 100,
    categories: null,
    originatingCurrency: "USD",
    recievingCurrency: "USD",
    items: null,
    originatingLocation: null,
    originatorAccount: "dummy-account",
    recieverAccount: "dummy-reciever-account",
    recievingLocation: null,
    sendingLocation: null,
    createdDate: new Date(),
    finalizedDate: null,
    status: "unknown",
    method: "card"
  };
  it('mounts', () => {
    cy.mount(<TransactionOverView transaction={dummyTransaction}/>)
  })
})