import React, { useState } from "react";
import { gql, useQuery } from "urql";
import { getStatesQuery } from "../lib/urql";
import {DisplayStatesAsTable} from "../components/displayStatesAsTable"



export default function ListStatesPage(props) {
  const [result, reexecuteQuery] = useQuery({ query: getStatesQuery });
  if (result.fetching) {
    return "please wait";
  } else if (result.data) {
    return (
      <DisplayStatesAsTable
        states={result.data.states}
        showId={false}
        showAbbreviation={true}
        showName={true}
      />
    );
  } else {
    return "fallthrough case";
  }
}
