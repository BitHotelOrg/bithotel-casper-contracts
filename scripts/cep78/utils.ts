import { CLKey, CLValueBuilder } from "casper-js-sdk";
import { Buffer } from "buffer";

export function stringToKey(string: string): CLKey {
  return CLValueBuilder.key(
    CLValueBuilder.byteArray(convertHashStrToHashBuff(string))
  );
}

export function convertHashStrToHashBuff(hashStr: string) {
  let hashHex = hashStr;
  if (hashStr.startsWith("hash-")) {
    hashHex = hashStr.slice(5);
  }
  return Buffer.from(hashHex, "hex");
}
