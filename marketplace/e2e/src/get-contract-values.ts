import { config } from "dotenv";
// config();
config({ path: ".env.test.local" });
import {
  Keys,
  CasperClient,
  CLValueBuilder,
  decodeBase16,
} from "casper-js-sdk";
import { CEP47Client } from "casper-cep47-js-client";
import { BigNumberish, parseFixed } from "@ethersproject/bignumber";
import {
  getAccountNamedKeyValue,
  getDeploy,
  getBinary,
  getAccountInfo,
} from "./utils";
import { MarketplaceClient } from "./clients/marketplace";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  MARKETPLACE_CONTRACT_NAME,
  CE47_CONTRACT_NAME,
  INSTALL_PAYMENT_AMOUNT,
  MARKETPLACE_CONTRACT,
  MINT_ONE_PAYMENT_AMOUNT,
  DEFAULT_ENTRYPOINT_PAYMENT_AMOUNT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const test = async () => {
  const marketplace = new MarketplaceClient(NODE_ADDRESS!, CHAIN_NAME!);
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  const accountInfo = await getAccountInfo(casperClient, KEYS.publicKey);

  console.log(accountInfo);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_hash`
  );
  const marketplaceContractPackageHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_package_hash`
  );

  const nftContractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${CE47_CONTRACT_NAME!}_contract_hash`
  );
  cep47.setContractHash(nftContractHash);

  marketplace.setContractHash(contractHash);

  const feeWallet = await marketplace.feeWallet();
  console.log(feeWallet);

  const tokenMeta = await cep47.getTokenMeta("1");

  console.log(tokenMeta);
};

test();
