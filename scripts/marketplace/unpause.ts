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

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const adminKey = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db"
);

const runtimeArgs = RuntimeArgs.fromMap({});

const preparedDeploy = contractClient.callEntrypoint(
  "un_pause",
  runtimeArgs,
  adminKey.publicKey,
  "casper-test",
  "5000000000",
  [adminKey]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
