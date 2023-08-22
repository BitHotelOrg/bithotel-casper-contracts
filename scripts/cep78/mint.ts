import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLString,
  CLKey,
  Keys,
} from "casper-js-sdk";
// import {
//   getKeysFromHexPrivKey,
//   SignatureAlgorithm,
// } from "casper-js-sdk/dist/lib/Keys";
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
const recipientKey = Keys.getKeysFromHexPrivKey(
  privateKey,
  Keys.SignatureAlgorithm.Ed25519
);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(
  "hash-59a13d7a9c89d989a13b9b15e183eb0ab695b95c98fd98a082b171efacd1c3ca"
);

const metadata = {
  name: "CasperPunk Hotel Guest",
  description:
    "The original and limited edition CasperPunk hotel guest that is only available on the Casper blockchain. This scary character is in high demand among hotel guests.",
  image:
    "https://apigateway.bithotel.io/metadata/QmSFjb5DtAg5Zyf6fvferRCGijUfLUmMV1jTYCX7k1uTRG",
  external_link: "https://www.bithotel.io/#/",
  attributes: [
    { trait_type: "Rarity", value: "Uncommon" },
    { trait_type: "Replicas", value: "3000" },
    { trait_type: "Drop", value: "Season 2" },
  ],
};

const runtimeArgs = RuntimeArgs.fromMap({
  token_owner: new CLKey(key.publicKey),
  token_meta_data: new CLString(JSON.stringify(metadata)),
});

const preparedDeploy = contractClient.callEntrypoint(
  "mint",
  runtimeArgs,
  key.publicKey,
  "casper-test",
  "5000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log);
