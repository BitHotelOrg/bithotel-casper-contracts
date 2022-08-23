import {
  CasperClient,
  CLPublicKey,
  Keys,
  CasperServiceByJsonRPC,
} from "casper-js-sdk";
import _ from "lodash";
import * as fs from "fs";

export const parseTokenMeta = (str: string): Array<[string, string]> =>
  str.split(",").map((s) => {
    const map = s.split(" ");
    return [map[0], map[1]];
  });

export const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

/**
 * Returns a set ECC key pairs - one for each NCTL user account.
 * @param {String} pathToUsers - Path to NCTL user directories.
 * @return {Array} An array of assymmetric keys.
 */
export const getKeyPairOfUserSet = (pathToUsers: string) => {
  return [1, 2, 3, 4, 5].map((userID) => {
    return Keys.Ed25519.parseKeyFiles(
      `${pathToUsers}/user-${userID}/public_key.pem`,
      `${pathToUsers}/user-${userID}/secret_key.pem`
    );
  });
};

export const getDeploy = async (NODE_URL: string, deployHash: string) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  while (i != 0) {
    const [deploy, raw] = await client.getDeploy(deployHash);
    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      } else {
        // @ts-ignore
        throw Error(
          "Contract execution: " +
            // @ts-ignore
            raw.execution_results[0].result.Failure.error_message
        );
      }
    } else {
      i--;
      await sleep(1000);
      continue;
    }
  }
  throw Error("Timeout after " + i + "s. Something's wrong");
};

interface AccountInfo {
  namedKeys: any;
}

export const getAccountInfo = async (
  client: CasperClient,
  publicKey: CLPublicKey
): Promise<AccountInfo> => {
  const accountHash = publicKey.toAccountHashStr();
  const stateRootHash = await client.nodeClient.getStateRootHash();
  const { Account: accountInfo } = await client.nodeClient.getBlockState(
    stateRootHash,
    accountHash,
    []
  );

  return accountInfo!;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param {CasperClient} client - JS SDK client for interacting with a node.
 * @param {Object} keyPair - Assymmetric keys of an on-chain account.
 * @param {String} namedKey - A named key associated with an on-chain account.
 * @return {String} On-chain account storage item value.
 */
export const getAccountNamedKeyValue = async (
  client: CasperClient,
  publicKey: CLPublicKey,
  namedKey: string
): Promise<string> => {
  // Chain query: get account information.
  const accountInfo = await getAccountInfo(client, publicKey);
  // console.log("accountInfo:", accountInfo);
  // Get value of contract v1 named key.
  const { key: contractHash } = _.find(accountInfo.namedKeys, (i) => {
    return i.name === namedKey;
  });

  return contractHash;
};
