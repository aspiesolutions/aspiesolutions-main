// this model should be desined to answer Who,What,When,Where,Why, and how
// Who performed the transaction
// Who recieved the transaction (if possible)
// Where did the money come from?
// When was the money spent
// When was the money recieve (if possible)
// Where was the money spent
// What was the money spent on
// How was the transaction paid?

// this model is to implement only functionallity required on the JS side
export class Transaction {
    // where did the money come from
    originatorId:string
    // who or what was the money sent to
    recieverId:string
    // who or what performed the transaction
    senderId:string  | null
    // where was the money sent from
    originatingLocation: string | null
    // where was the money sent to
    recievingLocation: string | null
    // where was the location of the person or thing who performed this transaction
    sendingLocation: string | null
    // what was the money spent on
    items: string | null
    // the total cost/gain of this transaction
    amount: number
    // what currency was the transaction performed in
    originatingCurrency:string
    // the currency of the reciever. used to allow conversion to/from external currencies
    recievingCurrency:string
    // what account the money was sent from
    originatorAccount:string
    // what account the money was sent to
    recieverAccount:string
    // used to organize transactions
    categories: string | null
}