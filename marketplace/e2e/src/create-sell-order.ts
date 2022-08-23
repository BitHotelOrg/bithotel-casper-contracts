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

const deployMarketplace = async () => {
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

  const tokenId = "44";

  try {
    const index = await cep47.getIndexByToken(KEYS.publicKey, tokenId);
    const owner = await cep47.getOwnerOf(tokenId);
    console.log({ index, owner, nftOwner: KEYS.publicKey.toAccountHashStr() });
  } catch (error: any) {
    const mintDeploy = await cep47.mint(
      KEYS.publicKey,
      [tokenId],
      [new Map([["number", "one"]])],
      MINT_ONE_PAYMENT_AMOUNT!,
      KEYS.publicKey,
      [KEYS]
    );

    console.log(`...... Minting ${tokenId}`);

    const mintDeployHash = await mintDeploy.send(NODE_ADDRESS!);

    console.log("...... Mint deploy hash: ", mintDeployHash);

    await getDeploy(NODE_ADDRESS!, mintDeployHash);
    console.log("...... Token minted successfully");
  }

  try {
    const allowance = await cep47.getAllowance(KEYS.publicKey, tokenId);
    console.log({ allowance });
  } catch (error: any) {
    const approveDeploy = await cep47.approve(
      CLValueBuilder.byteArray(
        decodeBase16(marketplaceContractPackageHash.slice(5))
      ),
      [tokenId],
      DEFAULT_ENTRYPOINT_PAYMENT_AMOUNT!,
      KEYS.publicKey,
      [KEYS]
    );

    console.log(
      `..... Aprrove ${tokenId} to ${marketplaceContractPackageHash}`
    );
    const approveDeployHash = await approveDeploy.send(NODE_ADDRESS!);
    console.log("...... Approve deploy hash: ", approveDeployHash);
    await getDeploy(NODE_ADDRESS!, approveDeployHash);
    console.log("...... Token approved successfully");
  }

  const formatedContractHash = `contract-${nftContractHash.slice(5)}`;
  const price = "50000000000";
  marketplace.setContractHash(contractHash);
  const tokens = new Map<BigNumberish, BigNumberish>([]);
  tokens.set(tokenId, price);
  const deploy = marketplace.createSellOrder(
    Date.now(),
    formatedContractHash,
    tokens,
    KEYS,
    "2500000000"
  );
  const createSellOrderDeployHash = await deploy.send(NODE_ADDRESS!);
  console.log({ createSellOrderDeployHash });
  await getDeploy(NODE_ADDRESS!, createSellOrderDeployHash);
  console.log({ createSellOrderDeployHash });
};

deployMarketplace();
