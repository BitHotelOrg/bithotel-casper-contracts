import { config } from "dotenv";
config({ path: ".env.test.local" });

import { ERC20Client } from "casper-erc20-js-client";
import { utils } from "casper-js-client-helper";
import { getDeploy } from "../utils";

import { Keys } from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  ERC20_WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  TOKEN_NAME,
  TOKEN_SYMBOL,
  TOKEN_DECIMALS,
  TOKEN_SUPPLY,
  ERC20_INSTALL_PAYMENT_AMOUNT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const test = async () => {
  const erc20 = new ERC20Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  const installDeployHash = await erc20.install(
    KEYS,
    TOKEN_NAME!,
    TOKEN_SYMBOL!,
    TOKEN_DECIMALS!,
    TOKEN_SUPPLY!,
    ERC20_INSTALL_PAYMENT_AMOUNT!,
    ERC20_WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `erc20_token_contract`
  );

  console.log(`... Contract Hash: ${contractHash}`);
};

test();
