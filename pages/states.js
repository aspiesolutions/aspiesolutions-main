import React, { useState } from "react";
import { gql, useQuery } from "urql";
import { client } from "../lib/urql";
import {DisplayStatesAsTable} from "../components/displayStatesAsTable"
const getStatesQuery = gql`
  query getStatesQuery {
    states {
      id
      name
      abbreviation
    }
  }
`;


export default function ListStatesPage(props) {
  const [result, rexecuteQuery] = useQuery({ query: getStatesQuery });
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
    return "hello world";
  }
}
