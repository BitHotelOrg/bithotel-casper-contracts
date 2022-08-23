import { config } from "dotenv";
config({ path: ".env.test.local" });
import { CEP47Client } from "casper-cep47-js-client";
import { getDeploy, getAccountNamedKeyValue, sleep } from "../utils";

import { Keys, CasperClient } from "casper-js-sdk";

import testData from "../test-data/nft.json";

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  CE47_CONTRACT_NAME,
  MINT_ONE_PAYMENT_AMOUNT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const test = async () => {
  console.log(KEYS.accountHex(), CE47_CONTRACT_NAME);
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  // const contractHash = await getAccountNamedKeyValue(
  //   casperClient,
  //   KEYS.publicKey,
  //   `${CE47_CONTRACT_NAME!}_contract_hash`
  // );

  // const contractPackageHash = await getAccountNamedKeyValue(
  //   casperClient,
  //   KEYS.publicKey,
  //   `${CE47_CONTRACT_NAME!}_contract_package_hash`
  // );

  const contractHash = `hash-3349d98a013dba40bc95e77ac7225c31657a6ee39786729f362366956816c9f1`;
  const contractPackageHash = `hash-33f0ba69adef5b8898ade783f1d8ad9386fb3b2cd41f44b6ff6cf165b000448a`;

  console.log(`... Contract Hash: ${contractHash}`);

  cep47.setContractHash(contractHash, contractPackageHash);

  const name = await cep47.name();
  console.log(`... Contract name: ${name}`);

  const symbol = await cep47.symbol();
  console.log(`... Contract symbol: ${symbol}`);

  const meta = await cep47.meta();
  console.log(`... Contract meta: ${JSON.stringify(meta)}`);

  let totalSupply = await cep47.totalSupply();
  console.log(`... Total supply: ${totalSupply}`);

  console.log("\n*************************\n");

  const promises = testData.tokens.map(async (token) => {
    console.log(`... Mint token ${token.tokenId} \n`);

    const meta = new Map<string, string>([]);

    Object.entries(token.trait).forEach((keyValue) => {
      meta.set(keyValue[0], keyValue[1]);
    });

    const mintDeploy = await cep47.mint(
      KEYS.publicKey,
      [`${token.tokenId}`],
      [meta],
      MINT_ONE_PAYMENT_AMOUNT!,
      KEYS.publicKey,
      [KEYS]
    );

    const mintDeployHash = await mintDeploy.send(NODE_ADDRESS!);

    console.log("...... Mint deploy hash: ", mintDeployHash);

    await getDeploy(NODE_ADDRESS!, mintDeployHash);
    console.log(`...... ${token.tokenId}  Token minted successfully`);
  });

  await Promise.all(promises);
};

test();
