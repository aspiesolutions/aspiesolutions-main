import React, { useState } from "react";
import { gql, useQuery } from "urql";
import { client } from "../lib/urql";

const getStatesQuery = gql`
  query getStatesQuery {
    states {
      id
      name
      abbreviation
    }
  }
`;
function StateNameTableData(props) {
  return <td>{props?.state?.name}</td>;
}
function StateTableRow(props) {
  return (
    <tr>
      <StateNameTableData state={props.state} />
    </tr>
  );
}
function DisplayStatesAsTable(props) {
  let [showId, setShowId] = useState(props?.showId || false);
  let [showAbbreviation, setShowAbbreviation] = useState(
    props?.showAbbreviation || false
  );
  let [showName, setShowName] = useState(props.showAbbreviation || false);
  return (
    <div>
      {/* table controls */}
      <form onSubmit={() => false}>
        <fieldset>
          <legend>Show/Hide Columns</legend>
          <label>
            <input
              id="show-id"
              type="checkbox"
              name="showId"
              checked={showId}
              onChange={() => {
                setShowId(!showId);
              }}
            />
            Id
          </label>
          <label>
            <input
              id="show-id"
              type="checkbox"
              name="showAbbreviation"
              checked={showAbbreviation}
              onChange={() => {
                setShowAbbreviation(!showAbbreviation);
              }}
            />
            Abbreviation
          </label>
          <label>
            <input
              id="show-name"
              type="checkbox"
              name="showName"
              checked={showName}
              onChange={()=>{setShowName(!showName)}}
            />
            Name
          </label>
        </fieldset>
      </form>
      <table>
        <thead>
          <tr>
            {/* display the 'id' table header */}
            {showId === true ? <th>Id</th> : null}
            {/* display the Abbreviation table header */}
            {showAbbreviation === true ? <th>Abbreviation</th> : null}
            {/* diplsay he name table header */}
            {showName === true ? <th>Name</th> : null}
          </tr>
        </thead>
        <tbody>
          {(props.states || []).map((state) => {
            return (
              <tr key={state.id}>
                {/* display the id column if enabled */}
                {showId === true ? <td>{state.id}</td> : null}
                {/* display the abbreviation column if enabled */}
                {showAbbreviation === true ? (
                  <td>{state.abbreviation}</td>
                ) : null}
                {/* display the name column if enabled */}
                {showName === true ? <td>{state.name}</td> : null}
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

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
