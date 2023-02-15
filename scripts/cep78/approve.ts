import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLString,
  CLU64,
} from "casper-js-sdk";
import { stringToKey } from "./utils";
import dotenv from "dotenv";

dotenv.config();

let rpcUri: string;

if (process.env.RPC_URI) {
  rpcUri = process.env.RPC_URI;
} else {
  throw new Error(`No rpcUri found`);
}

const casperClient = new CasperClient(rpcUri);

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-0c35ad2a4bbc8567308e16f97e826f68ddd1685cc9960e87d53024654a1ce41f"
);

const runtimeArgs = RuntimeArgs.fromMap({
  operator: stringToKey(
    "f9c09b8f327aff6fb82075f971d57b8c21e98f561eadd9e1540ccdca2dc36255"
  ),
  token_id: new CLU64(1),
});

const preparedDeploy = contractClient.callEntrypoint(
  "approve",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "8000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log);
