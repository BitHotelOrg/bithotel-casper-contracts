import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLU64,
} from "casper-js-sdk";
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

const key = Keys.getKeysFromHexPrivKey(
  privateKey,
  Keys.SignatureAlgorithm.Ed25519
);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db"
);

const runtimeArgs = RuntimeArgs.fromMap({
  listing_id: new CLU64(4),
});

const preparedDeploy = contractClient.callEntrypoint(
  "cancel_listing",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "35000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
