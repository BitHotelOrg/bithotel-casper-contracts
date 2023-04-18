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

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-b590bbf67fb2f60bdb54a7295bbb602f63008e4d8bb49fefe093ec805283b82f"
);

const runtimeArgs = RuntimeArgs.fromMap({
  listing_id: new CLU64(3),
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
