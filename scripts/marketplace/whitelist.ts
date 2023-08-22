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

if (process.env.RPC_URI && process.env.PRIVATE_KEY) {
  rpcUri = process.env.RPC_URI;
  privateKey = process.env.PRIVATE_KEY;
} else {
  throw new Error(`No rpcUri or privateKey found`);
}

const casperClient = new CasperClient(rpcUri);

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db"
);

const runtimeArgs = RuntimeArgs.fromMap({
  collection: CasperHelpers.stringToKey(
    "59a13d7a9c89d989a13b9b15e183eb0ab695b95c98fd98a082b171efacd1c3ca"
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
