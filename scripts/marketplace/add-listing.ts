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

const key2 = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

// b590bbf67fb2f60bdb54a7295bbb602f63008e4d8bb49fefe093ec805283b82f
// cc8d74f5cdd36bf926ebb47f57f6d6f2317846c852623fcee72bb5f756d99857
const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db"
);

const price = 75;

const runtimeArgs = RuntimeArgs.fromMap({
  collection: CasperHelpers.stringToKey(
    "59a13d7a9c89d989a13b9b15e183eb0ab695b95c98fd98a082b171efacd1c3ca"
  ),
  token_id: new CLU64(9),
  price: new CLU256(price * 1_000_000_000),
});

const preparedDeploy = contractClient.callEntrypoint(
  "add_listing",
  runtimeArgs,
  key2.publicKey,
  "casper-test",
  "10000000000",
  [key2]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
