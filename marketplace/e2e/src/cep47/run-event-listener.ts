import { config } from "dotenv";
config();
import { connect } from "mongoose";
import {
  CEP47Client,
  CEP47Events,
  CEP47EventParser,
} from "casper-cep47-js-client";
import {
  sleep,
  getDeploy,
  getAccountInfo,
  getAccountNamedKeyValue,
} from "../utils";

import {
  Keys,
  EventStream,
  EventName,
  CasperClient,
  CLMap,
  CLString,
  CLValueBuilder,
} from "casper-js-sdk";
import Asset from "../schema/asset.model";
import Collection from "../schema/collection.model";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,

  CE47_CONTRACT_NAME,
  CEP47_TOKEN_NAME,
  CEP47_TOKEN_SYMBOL,
  MONGODB_URL,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const startEventStream = async () => {
  const cep47 = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);

  const casperClient = new CasperClient(NODE_ADDRESS!);

  const contractHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${CE47_CONTRACT_NAME!}_contract_hash`
  );

  const contractPackageHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${CE47_CONTRACT_NAME!}_contract_package_hash`
  );

  let collection = await Collection.findOne({
    contractPackageHash: contractPackageHash.slice(5),
  });

  if (collection === null) {
    collection = new Collection({
      contractPackageHash: contractPackageHash.slice(5),
      contractHash: contractHash.slice(5),
      verified: true,
      slug: CE47_CONTRACT_NAME!,
      name: CEP47_TOKEN_NAME!,
      symbol: CEP47_TOKEN_SYMBOL!,
      description: "Casper NFT",
      image:
        "https://gateway.pinata.cloud/ipfs/QmahHrFUGaTRS53Dag6BQ68WRxnGVM7joCK8fDtsRB5QFB",
    });
    await collection.save();
  }

  console.log(`... Contract Hash: ${contractHash}`);

  cep47.setContractHash(contractHash, contractPackageHash);

  await sleep(5 * 1000);

  const es = new EventStream(EVENT_STREAM_ADDRESS!);

  es.subscribe(EventName.DeployProcessed, (event) => {
    const parsedEvents = CEP47EventParser(
      {
        contractPackageHash,
        eventNames: [CEP47Events.MintOne, CEP47Events.BurnOne],
      },
      event
    );

    if (parsedEvents && parsedEvents.success) {
      console.log("*** CEP47 EVENT ***");
      parsedEvents.data.map(async (event: any) => {
        const eventName = event.name as CEP47Events;
        const eventParams: CLMap<CLString, CLString> = event.clValue;
        const recipient = eventParams.get(CLValueBuilder.string("recipient"));
        const owner = eventParams.get(CLValueBuilder.string("owner"));
        const tokenId = eventParams.get(CLValueBuilder.string("token_id"));
        console.info(`Handling ${eventName} event`);

        switch (eventName) {
          case CEP47Events.MintOne:
            const asset = new Asset({
              collectionNFT: collection,
              tokenId: tokenId!.value(),
              name: `${collection!.name} #${tokenId!.value()}`,
              mintDate: Date.now(),
              metadata: "1:one",
            });
            console.log(asset);
            await asset.save();
            break;
        }
      });
      console.log("*** ***");
    }
  });

  es.start();
};

const storeEvent = async () => {
  try {
    await connect(MONGODB_URL!);
    console.log(`Connected to ${MONGODB_URL}`);
    startEventStream();
  } catch (err: any) {
    console.error(err);
  }
};
storeEvent();
