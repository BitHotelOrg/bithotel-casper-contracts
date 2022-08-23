import { config } from "dotenv";
config();
import {
  Keys,
  CasperClient,
  CLValueBuilder,
  decodeBase16,
  RuntimeArgs,
  CLKeyType,
} from "casper-js-sdk";
import {
  CasperContractClient,
  helpers,
  constants,
} from "casper-js-client-helper";
import { CEP47Client } from "casper-cep47-js-client";
import { BigNumberish, parseFixed } from "@ethersproject/bignumber";
import { getAccountNamedKeyValue, getDeploy, getBinary } from "./utils";
import { MarketplaceClient } from "./clients/marketplace";
import { Some, None } from "ts-results";
const {
  fromCLMap,
  toCLMap,
  installContract,
  setClient,
  contractSimpleGetter,
  contractCallFn,
  createRecipientAddress,
} = helpers;

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  MARKETPLACE_CONTRACT_NAME,
  CE47_CONTRACT_NAME,
  USER_KEY_PAIR_PATH,
  INSTALL_PAYMENT_AMOUNT,
  MARKETPLACE_CONTRACT,
  MINT_ONE_PAYMENT_AMOUNT,
  DEFAULT_ENTRYPOINT_PAYMENT_AMOUNT,
  PRE_ORDER_CSPR_CONTRACT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const KEYS_USER = Keys.Ed25519.parseKeyFiles(
  `${USER_KEY_PAIR_PATH}/public_key.pem`,
  `${USER_KEY_PAIR_PATH}/secret_key.pem`
);

const testCancelSellOrder = async () => {
  const casperClient = new CasperClient(NODE_ADDRESS!);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_hash`
  );
  const nftContractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${CE47_CONTRACT_NAME!}_contract_hash`
  );

  // const additionalRecipient = KEYS.publicKey;
  const additionalRecipient = undefined;

  console.log({ contractHash, nftContractHash });

  const runtimeArgs = RuntimeArgs.fromMap({
    marketplace_contract: CLValueBuilder.string(
      `contract-${contractHash.slice(5)}`
    ),
    entrypoint: CLValueBuilder.string("buy_sell_order_cspr"),
    collection: CLValueBuilder.string(`contract-${nftContractHash.slice(5)}`),
    token_id: CLValueBuilder.u256(41),
    amount: CLValueBuilder.u512("50000000000"),
    additional_recipient: additionalRecipient
      ? CLValueBuilder.option(Some(additionalRecipient))
      : CLValueBuilder.option(None, new CLKeyType()),
  });
  const deployHash = await installContract(
    CHAIN_NAME!,
    NODE_ADDRESS!,
    KEYS,
    runtimeArgs,
    "100000000000",
    PRE_ORDER_CSPR_CONTRACT!
  );
  console.log({ deployHash });
  await getDeploy(NODE_ADDRESS!, deployHash);
  console.log("done");
};

testCancelSellOrder();
