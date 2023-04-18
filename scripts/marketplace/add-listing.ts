import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLU64,
  CLU256,
} from "casper-js-sdk";
import { CasperHelpers } from "./helpers";
import dotenv from "dotenv";

dotenv.config();

let rpcUri: string;
let privateKey: string;

if (process.env.RPC_URI && process.env.PRIIVATE_KEY) {
  rpcUri = process.env.RPC_URI;
  privateKey = process.env.PRIIVATE_KEY;
} else {
  throw new Error(`No rpcUri or privateKey found`);
}
const casperClient = new CasperClient(rpcUri);

const key = Keys.getKeysFromHexPrivKey(
  privateKey,
  Keys.SignatureAlgorithm.Ed25519
);
// b590bbf67fb2f60bdb54a7295bbb602f63008e4d8bb49fefe093ec805283b82f
// cc8d74f5cdd36bf926ebb47f57f6d6f2317846c852623fcee72bb5f756d99857
const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-cc8d74f5cdd36bf926ebb47f57f6d6f2317846c852623fcee72bb5f756d99857"
);

console.log({ hash: contractClient.contractHash });

const runtimeArgs = RuntimeArgs.fromMap({
  collection: CasperHelpers.stringToKey(
    "0b659298a70a7bfcde35aceb49a0d7b4a34aed5e0a7e946db135d598edd411b9"
  ),
  token_id: new CLU64(2),
  price: new CLU256(50),
});

const preparedDeploy = contractClient.callEntrypoint(
  "add_listing",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "35000000000",
  [key]
);

// casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
