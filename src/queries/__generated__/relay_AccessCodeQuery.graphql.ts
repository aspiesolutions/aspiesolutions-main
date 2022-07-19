/**
 * @generated SignedSource<<19d1bed6a513d16f2344567e91216b7b>>
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
  } | null;
};
export type relay_AccessCodeQuery = {
  response: relay_AccessCodeQuery$data;
  variables: relay_AccessCodeQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": [
      {
        "kind": "Literal",
        "name": "id",
        "value": "abcdefg"
      }
    ],
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
    "storageKey": "accessCode(id:\"abcdefg\")"
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
    "cacheID": "fc3821544069b1289734a3fc8b0a2fa8",
    "id": null,
    "metadata": {},
    "name": "relay_AccessCodeQuery",
    "operationKind": "query",
    "text": "query relay_AccessCodeQuery {\n  accessCode(id: \"abcdefg\") {\n    id\n  }\n}\n"
  }
};
})();

(node as any).hash = "ee4cbb75c0edfffad446256b159d1656";

export default node;
