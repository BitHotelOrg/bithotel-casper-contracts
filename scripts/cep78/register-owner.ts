import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLString,
  CLKey,
  Keys,
  CLAccountHash,
} from "casper-js-sdk";
import dotenv from "dotenv";
import { CasperHelpers } from "../marketplace/helpers";

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

const recipientKey = Keys.getKeysFromHexPrivKey(
  privateKey,
  Keys.SignatureAlgorithm.Ed25519
);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-0b659298a70a7bfcde35aceb49a0d7b4a34aed5e0a7e946db135d598edd411b9"
);

const runtimeArgs = RuntimeArgs.fromMap({
  token_owner: CasperHelpers.stringToKey(
    "cc8d74f5cdd36bf926ebb47f57f6d6f2317846c852623fcee72bb5f756d99857"
  ),
});

const preparedDeploy = contractClient.callEntrypoint(
  "register_owner",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "1500000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log);
