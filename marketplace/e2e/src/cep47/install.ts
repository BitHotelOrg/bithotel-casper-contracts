import { config } from "dotenv";
// config();
config({ path: ".env.test.local" });
// config({ path: ".env.production.local" });
import { CEP47Client } from "casper-cep47-js-client";
import {
  parseTokenMeta,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
  getBinary,
} from "../utils";

import { Keys, CasperClient } from "casper-js-sdk";

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  CEP47_CONTRACT,
  MASTER_KEY_PAIR_PATH,
  CEP47_TOKEN_NAME,
  CE47_CONTRACT_NAME,
  CEP47_TOKEN_SYMBOL,
  CEP47_INSTALL_PAYMENT_AMOUNT,
} = process.env;

const TOKEN_META = new Map(parseTokenMeta(process.env.CEP47_TOKEN_META!));

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const test = async () => {
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const installDeployHash = cep47.install(
    getBinary(CEP47_CONTRACT!),
    {
      name: CEP47_TOKEN_NAME!,
      contractName: CE47_CONTRACT_NAME!,
      symbol: CEP47_TOKEN_SYMBOL!,
      meta: TOKEN_META,
    },
    CEP47_INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

  const hash = await installDeployHash.send(NODE_ADDRESS!);

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS!, hash);

  console.log(`... Contract installed successfully.`);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  let accountInfo = await getAccountInfo(casperClient, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${CE47_CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);
};

test();
