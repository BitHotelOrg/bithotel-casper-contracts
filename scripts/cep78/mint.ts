import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  Keys,
  CLString,
  CLKey,
} from "casper-js-sdk";
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

const metadata = {
  name: "Long Bookshelf",
  symbol: "LBS",
  token_uri:
    "https://apigateway.bithotel.io/metadata/QmWN9Gb4A1gT1j7DQj34gn3Mta3jQcsLmddjaU7JabcHuG",
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
  "8000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log);
