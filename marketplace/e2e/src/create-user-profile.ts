import User from "./schema/user.model";
import { config } from "dotenv";
import { connect } from "mongoose";
import { Keys } from "casper-js-sdk";
config();

const {
  EVENT_STREAM_ADDRESS,
  MONGODB_URL,
  NODE_ADDRESS,
  MARKETPLACE_CONTRACT_NAME,
  MASTER_KEY_PAIR_PATH,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const createUserProfile = async () => {
  await connect(MONGODB_URL!);
  const user = new User({
    name: "Test user",
    email: "test@gmail.com",
    id: "test",
    accountHash: KEYS.publicKey.toAccountHashStr(),
  });
  await user.save();
  console.log(user);
};

createUserProfile();
