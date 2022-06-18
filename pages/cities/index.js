import React from "react"
import { gql, useQuery } from "urql"

const listAllCitiesQuery = gql`
    query listAllCities {
        cities {
            id,
            name
        }
    }

`


export default function ListCitiesProps(props) {
    const [response] = useQuery({query:listAllCitiesQuery})
    if (response.fetching) {
        return "please wait"
    }
    if (response.error) {
        return "an error occured"
    }
    if (response.data) {
        return JSON.stringify(response.data)
    }

    return "Fallthrough case!"
}