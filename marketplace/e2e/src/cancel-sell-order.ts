import { config } from "dotenv";
config();
import {
  Keys,
  CasperClient,
  CLValueBuilder,
  decodeBase16,
} from "casper-js-sdk";
import { CEP47Client } from "casper-cep47-js-client";
import { BigNumberish, parseFixed } from "@ethersproject/bignumber";
import { getAccountNamedKeyValue, getDeploy, getBinary } from "./utils";
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

const testCancelSellOrder = async () => {
  const marketplace = new MarketplaceClient(NODE_ADDRESS!, CHAIN_NAME!);
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const casperClient = new CasperClient(NODE_ADDRESS!);

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

  const tokenId = "40";

  const formatedContractHash = `contract-${nftContractHash.slice(5)}`;

  marketplace.setContractHash(contractHash);
  const deploy = marketplace.cancelSellOrder(
    formatedContractHash,
    [tokenId],
    KEYS,
    "2500000000"
  );
  const cancelSellOrderDeployHash = await deploy.send(NODE_ADDRESS!);
  console.log({ cancelSellOrderDeployHash });
  await getDeploy(NODE_ADDRESS!, cancelSellOrderDeployHash);
  console.log({ cancelSellOrderDeployHash });
};

testCancelSellOrder();
