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
  CLAccountHash,
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

const publicKey =
  "010e31a03ea026a8e375653573e0120c8cb96699e6c9721ae1ea98f896e6576ac3";

const accountHash = new CLAccountHash(
  CLPublicKey.fromHex(publicKey).toAccountHash()
);

const runtimeArgs = RuntimeArgs.fromMap({
  account: CLValueBuilder.key(accountHash),
});

const preparedDeploy = contractClient.callEntrypoint(
  "remove_admin",
  runtimeArgs,
  key.publicKey,
  chainName,
  "1000000000",
  [key]
);

casperClient.putDeploy(preparedDeploy).then(console.log).catch(console.error);
