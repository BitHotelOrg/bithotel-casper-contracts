import { config } from "dotenv";
// config();
config({ path: ".env.test.local" });
import { Keys, CasperClient, CLPublicKey, CLKey } from "casper-js-sdk";
import { getAccountNamedKeyValue, getDeploy } from "./utils";
import { MarketplaceClient } from "./clients/marketplace";

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  MARKETPLACE_CONTRACT_NAME,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const deployMarketplace = async () => {
  const marketplace = new MarketplaceClient(NODE_ADDRESS!, CHAIN_NAME!);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_hash`
  );

  marketplace.setContractHash(contractHash);

  const feeWallet = CLPublicKey.fromHex(
    "012303609afa7da88d5a420a670e85a4213a380c11a3e1f733b71a3338c0de73d2"
  );

  const deploy = marketplace.setFeeWallet(feeWallet, KEYS, "2500000000");
  const deployHash = await deploy.send(NODE_ADDRESS!);
  console.log({ deployHash });
  await getDeploy(NODE_ADDRESS!, deployHash);
  console.log({ deployHash });
};

deployMarketplace();
