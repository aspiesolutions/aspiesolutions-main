/**
 * @generated SignedSource<<8f89189c277f5ea737f27eaca5d3f1be>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest, Query } from 'relay-runtime';
export type relay_AccessCodeQuery$variables = {};
export type relay_AccessCodeQuery$data = {
  readonly accessCode: {
    readonly id: string;
  };
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
    "concreteType": "AccessCode",
    "kind": "LinkedField",
    "name": "accessCode",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "id",
        "storageKey": null
      }
    ],
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
    "cacheID": "81e0d04df873c898d98e70231bc7c327",
    "id": null,
    "metadata": {},
    "name": "relay_AccessCodeQuery",
    "operationKind": "query",
    "text": "query relay_AccessCodeQuery {\n  accessCode {\n    id\n  }\n}\n"
  }
};
})();

(node as any).hash = "c06fed2539b24e2c4090e62afb4dac4e";

export default node;
