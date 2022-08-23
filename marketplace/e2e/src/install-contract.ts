import { config } from "dotenv";
// config();
config({ path: ".env.test.local" });
// config({ path: ".env.production.local" });
import { Keys, CasperClient, encodeBase16 } from "casper-js-sdk";
import { getAccountNamedKeyValue, getDeploy, getBinary } from "./utils";
import { MarketplaceClient } from "./clients/marketplace";

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  INSTALL_PAYMENT_AMOUNT,
  MARKETPLACE_CONTRACT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const deployMarketplace = async () => {
  const marketplace = new MarketplaceClient(NODE_ADDRESS!, CHAIN_NAME!);
  const contractName = "kunft_marketplace";
  const acceptableTokens = new Map<string, number>([]);
  const null_contract_hash = new Uint8Array(32).fill(0);
  acceptableTokens.set(`contract-${encodeBase16(null_contract_hash)}`, 1000);
  const contractPackageHash = `contract-package-wasm7d7896b8e0f04f6c54904f834427be1fe8b0f5acdadb27e32c44051e35cb193c`;
  // const contractPackageHash = undefined;
  const deploy = marketplace.install(
    getBinary(MARKETPLACE_CONTRACT!),
    {
      feeWallet: KEYS.publicKey,
      contractName,
      acceptableTokens,
      contractPackageHash,
    },
    INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

  const installDeployHash = await deploy.send(NODE_ADDRESS!);

  console.log({ installDeployHash });

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... installed successfully.`);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${contractName}_contract_hash`
  );

  console.log({ contractHash });
};

deployMarketplace();
