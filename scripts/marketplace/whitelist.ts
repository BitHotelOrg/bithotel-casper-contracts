import { CasperClient, Contracts, RuntimeArgs, Keys } from "casper-js-sdk";
import { CasperHelpers } from "./helpers";
import dotenv from "dotenv";
import { getEnvironmentVars } from "./env";

dotenv.config();

const vars = getEnvironmentVars("prod");

const casperClient = new CasperClient(vars.rpcUri);

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(vars.marketplace);

const runtimeArgs = RuntimeArgs.fromMap({
  collection: CasperHelpers.stringToKey(vars.nft.split("-")[1]),
});

const preparedDeploy = contractClient.callEntrypoint(
  "whitelist",
  runtimeArgs,
  key.publicKey,
  "casper",
  "500000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
