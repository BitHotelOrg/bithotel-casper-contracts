import { CLKey, CLKeyParameters, CLValueBuilder } from "casper-js-sdk";

export class CasperHelpers {
  static stringToKey(string: string): CLKey {
    return CLValueBuilder.key(this.stringToKeyParameter(string));
  }

  static stringToKeyParameter(string: string): CLKeyParameters {
    return CLValueBuilder.byteArray(this.convertHashStrToHashBuff(string));
  }

  static convertHashStrToHashBuff(hashStr: string) {
    let hashHex = hashStr;
    if (hashStr.startsWith("hash-")) {
      hashHex = hashStr.slice(5);
    }
    return Buffer.from(hashHex, "hex");
  }
}
