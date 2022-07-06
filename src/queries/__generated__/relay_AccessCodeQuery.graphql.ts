/**
 * @generated SignedSource<<09cfc70ee46c272f4be81128e14b2439>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Query } from 'relay-runtime';
export type relay_AccessCodeQuery$variables = {};
export type relay_AccessCodeQuery$data = {
  readonly accessCode: string;
};
export type relay_AccessCodeQuery = {
  response: relay_AccessCodeQuery$data;
  variables: relay_AccessCodeQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "kind": "ScalarField",
    "name": "accessCode",
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "relay_AccessCodeQuery",
    "selections": (v0/*: any*/),
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "relay_AccessCodeQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "ff30a3978f6a4cb2cc3258397f26a067",
    "id": null,
    "metadata": {},
    "name": "relay_AccessCodeQuery",
    "operationKind": "query",
    "text": "query relay_AccessCodeQuery {\n  accessCode\n}\n"
  }
};
})();

(node as any).hash = "cc88ab6efbbb928fc18f66d36ffc179b";

export default node;
