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

  const token =
    `hash-c87426ed269f686554d63684834a49477c0e399c4d94cb2c26638025e2b5cc2a`.slice(
      5
    );
  const fee = 500;

  const deploy = await marketplace.setAcceptableToken(
    token,
    fee,
    KEYS.publicKey,
    "2500000000",
    [KEYS]
  );
  const deployHash = await deploy.send(NODE_ADDRESS!);
  console.log({ deployHash });
  await getDeploy(NODE_ADDRESS!, deployHash);
  console.log({ deployHash });
};

deployMarketplace();
