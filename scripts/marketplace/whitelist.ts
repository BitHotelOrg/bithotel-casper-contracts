import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLString,
  CLKey,
  Keys,
  CLValueBuilder,
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

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-cc8d74f5cdd36bf926ebb47f57f6d6f2317846c852623fcee72bb5f756d99857"
);

const runtimeArgs = RuntimeArgs.fromMap({
  collection: CasperHelpers.stringToKey(
    "0b659298a70a7bfcde35aceb49a0d7b4a34aed5e0a7e946db135d598edd411b9"
  ),
});

const preparedDeploy = contractClient.callEntrypoint(
  "whitelist",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "8000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
