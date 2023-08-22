import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLU64,
  CLU512,
} from "casper-js-sdk";
import dotenv from "dotenv";
import { readFileSync } from "fs";
import { CasperHelpers } from "./helpers";

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

const executeListingFile = readFileSync("./execute_listing_call.wasm");

const wasm = Uint8Array.from(executeListingFile);

const contractClient = new Contracts.Contract();

const price = 27;

const runtimeArgs = RuntimeArgs.fromMap({
  marketplace_contract_hash: CasperHelpers.stringToKey(
    "411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db"
  ),
  listing_id: new CLU64(16),
  amount: new CLU512(price * 1_000_000_000),
});

const preparedDeploy = contractClient.install(
  wasm,
  runtimeArgs,
  "15000000000",
  key.publicKey,
  "casper-test",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
