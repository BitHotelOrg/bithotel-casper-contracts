import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLString,
  CLKey,
  Keys,
  CLValueBuilder,
  CLKeyBytesParser,
  CLPublicKey,
  CLValue,
  CLType,
  CLKeyType,
} from "casper-js-sdk";
import { CasperHelpers } from "./helpers";
import { getEnvironmentVars } from "../env";
import dotenv from "dotenv";

dotenv.config();

const { rpcUri, marketplace, chainName } = getEnvironmentVars();

const casperClient = new CasperClient(rpcUri);

const privateKeyPath = "/Users/bufo/Downloads/BitHotel_secret_key.pem";

const key = Keys.Ed25519.loadKeyPairFromPrivateFile(privateKeyPath);

const contractClient = new Contracts.Contract();
contractClient.setContractHash(marketplace);

const runtimeArgs = RuntimeArgs.fromMap({
  account: CasperHelpers.stringToKey(
    "821266406b21b68b4f698efd03df624287e4c195503bc3d9f107a3d829ae6edf"
  ),
});

const preparedDeploy = contractClient.callEntrypoint(
  "add_admin",
  runtimeArgs,
  key.publicKey,
  chainName,
  "5000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
