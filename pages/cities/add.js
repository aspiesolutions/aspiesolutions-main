import React, { useCallback, useMemo, useState } from "react";
import { useQuery, gql, useMutation } from "urql";
import _ from "lodash";

const addCityGetStatesQuery = gql`
  query addCityGetStatesQuery {
    states {
      id
      name
    }
  }
`;
const addCityGetCitesQuery = gql`
  query addCityGetCitiesQuery($state_id: String) {
    cities(where: { state_id: { equals: $state_id } }) {
      id
      name
    }
  }
`;

const addManyCitiesToStateMutation = gql`
  mutation addManyCitiesToStateMutation($cities: [CityCreateManyInput!]!) {
    createManyCity(data: $cities) {
      count
    }
  }
`;

export default function AddCity(props) {
  let [addCityGetStatesResult, reexecuteAddCityGetStatesQuery] = useQuery({
    query: addCityGetStatesQuery,
  });
  let [state, setState] = useState(null);
  let [addCityGetCitiesResult, reexecuteAddCityGetCitiesQuery] = useQuery({
    query: addCityGetCitesQuery,
    variables: { state_id: state?.id },
  });
  let [addCityMutationResult, addCity] = useMutation({
    query: addManyCitiesToStateMutation,
  });
  // optimize the selectStateChangeHandler
  const selectStateChangeHandler = useCallback(
    (e) => {
      let stateFromStateName =
        addCityGetStatesResult.data.states.find(
          (state) => e.target.value === state.name
        ) || null;
      setState(stateFromStateName);
    },
    [addCityGetStatesResult?.data?.states, setState]
  );
  const addCitySubmitFormCallback = useCallback(
    (formSubmitEvent) => {
      formSubmitEvent.preventDefault();
      // get the textarea associated with this form
      let textarea = formSubmitEvent.currentTarget.querySelector(
        `textarea[name="cities"]`
      );
      if (textarea == null) {
        // stop handling this event if we can't find the textarea
        return false;
      }
      // copy the value into local memory
      let citiesString = textarea.value;
      // release the handle to the element
      textarea = null;
      let citiesList = citiesString.split("\n");
      // Trim each string, removing empty space
      citiesList = citiesList.map((city) => city.trim());
      // dont try to add cities that already exist
      citiesList = citiesList.filter(
        (city) => (addCityGetCitiesResult?.data?.cites || []).includes(city) === false
      );
      // convert the list of strings into input objects
      let citiesInputObjects = citiesList.map((city) => {
        return { name: city, state_id: state.id };
      });
      citiesList = null;
      // start the process

      console.log(citiesInputObjects);
    },
    [addCityGetCitiesResult?.data?.cities, state, addCityMutationResult]
  );
  // schedule a
  if (addCityGetStatesResult.fetching) {
    return "please wait";
  }
  if (addCityGetStatesResult.error) {
    return "failed to get the list of states";
  }
  return (
    <div>
      <form onSubmit={(e) => false}>
        <label htmlFor="select-state">Select State</label>
        <input
          required
          list="states"
          id="select-state"
          name="states"
          onChange={selectStateChangeHandler}
        />
        <datalist id="states">
          {addCityGetStatesResult.data.states.map((state) => (
            <option key={state.id} value={state.name} />
          ))}
        </datalist>
      </form>
      {addCityGetCitiesResult.fetching ? "Getting Cities, Please wait" : null}
      {state == null ? "Invalid or non existant state" : null}
      {state != null &&
      addCityGetCitiesResult.fetching === false &&
      typeof addCityGetCitiesResult?.data?.cities === "object" ? (
        <>
          <form onSubmit={addCitySubmitFormCallback}>
            <label htmlFor="addCitiesInput">
              <p>
                To Add one or more cities to the current state, enter each state
                in the text box below.
              </p>
              <p>
                Enter multiple states by pressing the enter key between states,
                or paste a list of states seperated by newlines
              </p>
            </label>
            <br />
            <textarea id="addCitiesInput" name="cities"></textarea>
            <br />
            <button type="submit">submit</button>
          </form>
        </>
      ) : null}
    </div>
  );
}
