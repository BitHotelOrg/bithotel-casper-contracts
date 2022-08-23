import { config } from "dotenv";
config();
import {
  Keys,
  CasperClient,
  CLValueBuilder,
  encodeBase16,
  signFormattedMessage,
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  MASTER_KEY_PAIR_PATH,
  INSTALL_PAYMENT_AMOUNT,
  MARKETPLACE_CONTRACT,
} = process.env;

const private_key = Keys.Ed25519.parsePrivateKeyFile(
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
const public_key = Keys.Ed25519.privateToPublicKey(private_key);

const KEYS = Keys.Ed25519.parseKeyPair(public_key, private_key);

const message = new TextEncoder().encode(
  `Signup with KUNFT with ${KEYS.publicKey.toHex()}`
);

const signedMessage = KEYS.sign(message);

console.log("publickey", KEYS.publicKey.toHex());
console.log("message:", encodeBase16(message));
console.log("signature", encodeBase16(signedMessage));

const verified = KEYS.verify(signedMessage, message);

const sig = signFormattedMessage(KEYS, message);

console.log("sig2", encodeBase16(sig));

console.log(verified);
// while (true);

// 0146c64d0506c486f2b19f9cf73479fba550f33227b6ec1c12e58b437d2680e96d
// 5369676e75702077697468204b554e4654207769746820303134366336346430353036633438366632623139663963663733343739666261353530663333323237623665633163313265353862343337643236383065393664
// a625bf7f143acdfb058cdbd6899ea95760dfdabd1d84e937313de3fd8f66432014654fe1271b4020441ddec102ed6491462191107e2f084442ccd35f34701905
