export async function querySmartContract(url: string, contractAddr: string, query: string) {
  const query_encoded = btoa(query);
  const request = `${url}/cosmwasm/wasm/v1/contract/${contractAddr}/smart/${query_encoded}`
  return await fetch(request).then((res) => res.json());
}

// export async function executeWasm(url: string, contract: string) {
//
// }
