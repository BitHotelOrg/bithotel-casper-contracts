import { config } from "dotenv";
config();
import { connect } from "mongoose";
import {
  CasperClient,
  CLByteArray,
  CLMap,
  CLOption,
  CLString,
  CLStringType,
  CLU256,
  CLValue,
  CLValueBuilder,
  EventName,
  EventStream,
  Keys,
} from "casper-js-sdk";
import {
  MarketplaceEventParser,
  MarketplaceEvents,
} from "./clients/marketplace";
import {
  CEP47Client,
  CEP47Events,
  CEP47EventParser,
} from "casper-cep47-js-client";
import { getAccountNamedKeyValue } from "./utils";
import BuyOrder from "./schema/buyoder.model";
import SellOrder from "./schema/sellOrder.model";
import Collection from "./schema/collection.model";
import Asset from "./schema/asset.model";
import User from "./schema/user.model";

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  EVENT_STREAM_ADDRESS,
  MONGODB_URL,
  MARKETPLACE_CONTRACT_NAME,
  MASTER_KEY_PAIR_PATH,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const startEventStream = async () => {
  const es = new EventStream(EVENT_STREAM_ADDRESS!);
  const casperClient = new CasperClient(NODE_ADDRESS!);
  const contractPackageHash = await getAccountNamedKeyValue(
    casperClient,
    KEYS.publicKey,
    `${MARKETPLACE_CONTRACT_NAME!}_contract_package_hash`
  );

  console.log({ contractPackageHash });

  es.subscribe(EventName.DeployProcessed, async (event) => {
    const parsedEvents = MarketplaceEventParser(
      {
        contractPackageHash: contractPackageHash.slice(5),
        eventNames: [
          MarketplaceEvents.SellOrderCreated,
          MarketplaceEvents.SellOrderCanceled,
          MarketplaceEvents.SellOrderBought,
          MarketplaceEvents.BuyOrderCreated,
          MarketplaceEvents.BuyOrderCanceled,
          MarketplaceEvents.BuyOrderAccepted,
        ],
      },
      event
    );
    if (parsedEvents && parsedEvents.success) {
      console.log("***  MARKETPLACE EVENT  ***");

      const promises = parsedEvents.data.map(async (event: any) => {
        const eventName = event.name as MarketplaceEvents;
        const eventParams: CLMap<CLString, CLString> = event.clValue;
        console.info(`Handling ${eventName} event`);
        const creator = eventParams.get(CLValueBuilder.string("creator"));
        const collection = eventParams.get(CLValueBuilder.string("collection"));
        const tokenId = eventParams.get(CLValueBuilder.string("token_id"));
        const payToken = eventParams.get(CLValueBuilder.string("pay_token"));
        const price = eventParams.get(CLValueBuilder.string("price"));
        const startTime = eventParams.get(CLValueBuilder.string("start_time"));
        const buyer = eventParams.get(CLValueBuilder.string("buyer"));
        const additionalRecipient = eventParams.get(
          CLValueBuilder.string("additional_recipient")
        );
        let collectionDB = await Collection.findOne({
          contractHash: collection!.value(),
        });
        const cep47Client = new CEP47Client(NODE_ADDRESS!, CHAIN_NAME!);
        cep47Client.setContractHash(`hash-${collection!.value()}`);
        if (collectionDB === null) {
          const name = await cep47Client.name();
          console.log(
            `Creating ${name} collection for ${collection!.value()} contract hash.`
          );
          const symbol = await cep47Client.symbol();
          collectionDB = new Collection({
            contractHash: collection!.value(),
            slug: collection!.value(),
            name,
            symbol,
            verified: false,
          });
          await collectionDB.save();
        }
        const asset = await Asset.findOne({
          collectionNFT: collectionDB,
          tokenId: tokenId!.value(),
        });
        if (asset === null) {
          console.log(collection!.value());
          throw Error("Not exist token");
        }
        let formatedCreatorHash = creator!.value();
        formatedCreatorHash = formatedCreatorHash.slice(20).slice(0, -2);
        formatedCreatorHash = `account-hash-${formatedCreatorHash}`;
        switch (eventName) {
          case MarketplaceEvents.SellOrderCreated: {
            const sellOrder = new SellOrder({
              creator: formatedCreatorHash,
              asset,
              payToken:
                payToken!.value() === "None" ? undefined : payToken!.value(),
              price: price!.value(),
              startTime: startTime!.value(),
              status: "pending",
            });
            await sellOrder.save();
            break;
          }
          case MarketplaceEvents.SellOrderCanceled: {
            await SellOrder.findOneAndUpdate(
              {
                creator: formatedCreatorHash,
                asset,
                startTime: startTime!.value(),
              },
              { status: "canceled" }
            );
            break;
          }
          case MarketplaceEvents.SellOrderBought: {
            await SellOrder.findOneAndUpdate(
              {
                creator: formatedCreatorHash,
                asset,
                startTime: startTime!.value(),
              },
              { status: "succeed" }
            );
            break;
          }
          case MarketplaceEvents.BuyOrderCreated: {
            const owner = await cep47Client.getOwnerOf(tokenId!.value());
            const buyOrder = new BuyOrder({
              creator: formatedCreatorHash,
              asset,
              owner,
              payToken: payToken!.value(),
              price: price!.value(),
              startTime: startTime!.value(),
              additionalRecipient: additionalRecipient!.value(),
              status: "pending",
            });
            await buyOrder.save();
            break;
          }
          case MarketplaceEvents.BuyOrderCanceled: {
            BuyOrder.findOneAndUpdate(
              {
                creator: formatedCreatorHash,
                asset,
                startTime: startTime!.value(),
              },
              {
                status: "canceled",
              }
            );
            break;
          }
          case MarketplaceEvents.BuyOrderAccepted: {
            BuyOrder.findOneAndUpdate(
              {
                creator: formatedCreatorHash,
                asset,
                startTime: startTime!.value(),
              },
              {
                status: "succeed",
              }
            );
            break;
          }
          default:
            console.error(`Unhandled event: ${eventName}`);
        }
      });
      await Promise.all(promises);
      console.log("***     ***");
    }
  });
  es.start(0);
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
