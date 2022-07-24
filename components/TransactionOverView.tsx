// this component should be desined to answer Who,What,When,Where,Why, and how
// Who performed the transaction
// Who recieved the transaction (if possible)
// Where did the money come from?
// When was the money spent
// When was the money recieve (if possible)
// Where was the money spent
// What was the money spent on
// How was the transaction paid?

import React from "react";
import { Transaction } from "../src/lib/models/Transaction";

export interface TransactionOverViewProps {
  className?: string;
  transaction: Transaction;
}

export default function TransactionOverView(
  props: TransactionOverViewProps
): JSX.Element {
  let classNameDefault = props.className || "";
  let className = `transaction ${classNameDefault}`.trim();
  return (
    <div className={className}>
      <div className="id-group">
        <div className="id-label">id</div>
        <div className="id">{props.transaction.id}</div>
      </div>
      <div className="originator-id-group">
        <span className="originator-id-label">originator id</span>&nbsp;
        <span className="originator-id">{props.transaction.originatorId}</span>
      </div>
      <div className="reciever-id-group">
        <span className="reciever-id-label">reciever id</span>&nbsp;
        <span className="reciever-id">{props.transaction.recieverId}</span>
      </div>
      <div className="sender-id-group">
        <span className="sender-id-label">sender id</span>&nbsp;
        <div className="sender-id">{props.transaction.senderId}</div>
      </div>
      <div className="originating-location-group">
        <span className="originating-location-label">originating location</span>&nbsp;
        <span className="originating-location">
          {props.transaction.originatingLocation || "none"}
        </span>
      </div>
      <div className="recieving-location-group">
        <span className="recieving-location-label">recieving location</span>&nbsp;
        <span className="receiving-location">
          {props.transaction.recievingLocation || "null"}
        </span>
      </div>
      <div className="sending-location-group">
        <span className="sending-location-label">originating location</span>&nbsp;
        <span className="sending-location">
          {props.transaction.sendingLocation || "null"}
        </span>
      </div>
      <div className="items-group">
        <span className="items-label">items</span>&nbsp;
        <span className="items">{props.transaction.items || "null"}</span>
      </div>

      <div className="amount-group">
        <span className="amount-label">amount</span>&nbsp;
        <span className="amount">{props.transaction.amount}</span>
      </div>
      <div className="originating-currency">
        {props.transaction.originatingCurrency}
      </div>
      <div className="recieving-currency">
        {props.transaction.recievingCurrency}
      </div>
      <div className="originator-account">
        {props.transaction.originatorAccount}
      </div>
      <div className="reciever-account">
        {props.transaction.recieverAccount}
      </div>
      <div className="categories">{props.transaction.categories}</div>
      <div className="created-date">
        {props.transaction.createdDate.toUTCString()}
      </div>
      <div className="finalized-date">
        {props.transaction.finalizedDate?.toUTCString() || "null"}
      </div>
      <div className="status">{props.transaction.status}</div>
      <div className="method">{props.transaction.method}</div>
    </div>
  );
}
