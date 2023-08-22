import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLString,
  CLU64,
  CLKey,
  CLByteArray,
} from "casper-js-sdk";
import { stringToKey } from "./utils";
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

// 0165e406c81af68793a4f56b60f646f9eeba2fad1bd16f06cd1c42f6f8d88cc5fb
const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

// 013cafb1912c0ca0dc6e0251905f29ebe01176371c298e513a24c0f2d9b2bbff28
const key2 = Keys.getKeysFromHexPrivKey(
  privateKey,
  Keys.SignatureAlgorithm.Ed25519
);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-59a13d7a9c89d989a13b9b15e183eb0ab695b95c98fd98a082b171efacd1c3ca"
);

const runtimeArgs = RuntimeArgs.fromMap({
  operator: new CLKey(
    new CLByteArray(
      Uint8Array.from(
        Buffer.from(
          "411f0c5ae537b18beabfe43601d3e2d238bd7b89e1566ff78fff66a07826a0db",
          "hex"
        )
      )
    )
  ),
  token_id: new CLU64(9),
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
