import {
  CLValue,
  CLPublicKey,
  CLKey,
  CLMap,
  RuntimeArgs,
  CasperClient,
  Contracts,
  Keys,
  CLKeyParameters,
  CLValueBuilder,
  CLValueParsers,
  CLTypeTag,
  CLStringType,
  CLKeyType,
  encodeBase16,
  CLU64,
  CLString,
  CLU64Type,
  CLU32Type,
  CLU256Type,
} from "casper-js-sdk";
import { BigNumberish } from "@ethersproject/bignumber";
import { Some, None } from "ts-results";
import { types } from "casper-js-client-helper";

const { Contract, toCLMap, fromCLMap } = Contracts;
type RecipientType = types.RecipientType;

export interface MarketplaceInstallArgs {
  feeWallet: RecipientType;
  acceptableTokens: Map<string, number>;
  contractName: string;
  contractPackageHash?: string;
}

export enum MarketplaceEvents {
  SellOrderCreated = "SellOrderCreated",
  SellOrderCanceled = "SellOrderCanceled",
  SellOrderBought = "SellOrderBought",
  BuyOrderCreated = "BuyOrderCreated",
  BuyOrderCanceled = "BuyOrderCanceled",
  BuyOrderAccepted = "BuyOrderAccepted",
}

export const MarketplaceEventParser = (
  {
    contractPackageHash,
    eventNames,
  }: { contractPackageHash: string; eventNames: string[] },
  value: any
) => {
  if (value.body.DeployProcessed.execution_result.Success) {
    const { transforms } =
      value.body.DeployProcessed.execution_result.Success.effect;

    const cep47Events = transforms.reduce((acc: any, val: any) => {
      if (
        val.transform.hasOwnProperty("WriteCLValue") &&
        typeof val.transform.WriteCLValue.parsed === "object" &&
        val.transform.WriteCLValue.parsed !== null
      ) {
        const maybeCLValue = CLValueParsers.fromJSON(
          val.transform.WriteCLValue
        );
        const clValue = maybeCLValue.unwrap();
        if (clValue && clValue.clType().tag === CLTypeTag.Map) {
          const hash = (clValue as CLMap<CLValue, CLValue>).get(
            CLValueBuilder.string("contract_package_hash")
          );
          const preferContractPackageHash = contractPackageHash.startsWith(
            "hash-"
          )
            ? contractPackageHash.slice(5).toLowerCase()
            : contractPackageHash.toLowerCase();
          const event = (clValue as CLMap<CLValue, CLValue>).get(
            CLValueBuilder.string("event_type")
          );
          if (
            hash &&
            // NOTE: Calling toLowerCase() because current JS-SDK doesn't support checksumed hashes and returns all lower case value
            // Remove it after updating SDK
            hash.value() === preferContractPackageHash &&
            event &&
            eventNames.includes(event.value())
          ) {
            acc = [...acc, { name: event.value(), clValue }];
          }
        }
      }
      return acc;
    }, []);

    return { error: null, success: !!cep47Events.length, data: cep47Events };
  }

  return null;
};

export class MarketplaceClient {
  casperClient: CasperClient;
  contractClient: Contracts.Contract;

  constructor(public _nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(_nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

  public install(
    wasm: Uint8Array,
    args: MarketplaceInstallArgs,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys?: Keys.AsymmetricKey[]
  ) {
    const acceptable_tokens = new CLMap([new CLStringType(), new CLU32Type()]);
    Array.from(args.acceptableTokens.entries()).forEach((entry) => {
      acceptable_tokens.set(
        CLValueBuilder.string(entry[0]),
        CLValueBuilder.u32(entry[1])
      );
    });
    const runtimeArgs = RuntimeArgs.fromMap({
      fee_wallet: new CLKey(args.feeWallet),
      acceptable_tokens,
      contract_name: CLValueBuilder.string(args.contractName),
      contract_package_hash: args.contractPackageHash
        ? CLValueBuilder.option(
            Some(CLValueBuilder.string(args.contractPackageHash))
          )
        : CLValueBuilder.option(None, new CLStringType()),
    });
    return this.contractClient.install(
      wasm,
      runtimeArgs,
      paymentAmount,
      deploySender,
      this.networkName,
      keys || []
    );
  }

  public setContractHash(contractHash: string, contractPackageHash?: string) {
    this.contractClient.setContractHash(contractHash, contractPackageHash);
  }

  public createSellOrder(
    startTime: number,
    collection: string,
    tokens: Map<BigNumberish, BigNumberish>,
    key: Keys.AsymmetricKey,
    paymentAmount: string,
    payToken?: string
  ) {
    const tokensMap = new CLMap([new CLU256Type(), new CLU256Type()]);
    Array.from(tokens.entries()).forEach((token) => {
      tokensMap.set(
        CLValueBuilder.u256(token[0]),
        CLValueBuilder.u256(token[1])
      );
    });

    const runtimeArgs = RuntimeArgs.fromMap({
      start_time: CLValueBuilder.u64(startTime),
      collection: CLValueBuilder.string(collection),
      tokens: tokensMap,
      pay_token: payToken
        ? CLValueBuilder.option(Some(CLValueBuilder.string(payToken)))
        : CLValueBuilder.option(None, new CLStringType()),
    });

    return this.contractClient.callEntrypoint(
      "create_sell_order",
      runtimeArgs,
      key.publicKey,
      this.networkName,
      paymentAmount,
      [key]
    );
  }
  public cancelSellOrder(
    collection: string,
    tokenIds: BigNumberish[],
    key: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      collection: CLValueBuilder.string(collection),
      token_ids: CLValueBuilder.list(
        tokenIds.map((tokenId) => CLValueBuilder.u256(tokenId))
      ),
    });
    return this.contractClient.callEntrypoint(
      "cancel_sell_order",
      runtimeArgs,
      key.publicKey,
      this.networkName,
      paymentAmount,
      [key]
    );
  }

  public buySellOrder(
    collection: string,
    tokenId: BigNumberish,
    amount: BigNumberish,
    key: Keys.AsymmetricKey,
    paymentAmount: string,
    additionalReccipient?: CLKeyParameters
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      collection: CLValueBuilder.string(collection),
      token_id: CLValueBuilder.u256(tokenId),
      amount: CLValueBuilder.u256(amount),
      additional_recipient: additionalReccipient
        ? CLValueBuilder.option(Some(additionalReccipient))
        : CLValueBuilder.option(None, new CLKeyType()),
    });
    return this.contractClient.callEntrypoint(
      "buy_sell_order",
      runtimeArgs,
      key.publicKey,
      this.networkName,
      paymentAmount,
      [key]
    );
  }

  public async feeWallet() {
    const result = (await this.contractClient.queryContractData([
      "fee_wallet",
    ])) as CLValue;
    return encodeBase16(result.value());
  }

  public setFeeWallet(
    feeWallet: CLKeyParameters,
    key: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      fee_wallet: new CLKey(feeWallet),
    });
    return this.contractClient.callEntrypoint(
      "set_fee_wallet",
      runtimeArgs,
      key.publicKey,
      this.networkName,
      paymentAmount,
      [key]
    );
  }

  public createBuyOrder(
    collection: string,
    tokenId: BigNumberish,
    amount: BigNumberish,
    payToken: string,
    key: Keys.AsymmetricKey,
    paymentAmount: string,
    additionalReccipient?: CLKeyParameters
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      collection: CLValueBuilder.string(collection),
      token_id: CLValueBuilder.u256(tokenId),
      amount: CLValueBuilder.u256(amount),
      additional_recipient: additionalReccipient
        ? CLValueBuilder.option(Some(additionalReccipient))
        : CLValueBuilder.option(None, new CLKeyType()),
    });
    return this.contractClient.callEntrypoint(
      "cancel_sell_order",
      runtimeArgs,
      key.publicKey,
      this.networkName,
      paymentAmount,
      [key]
    );
  }

  public async setAcceptableToken(
    contractHash: string,
    fee: number,
    sender: CLPublicKey,
    paymentAmount: string,
    signingKeys?: Keys.AsymmetricKey[]
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      contract_hash: CLValueBuilder.string(`contract-${contractHash}`),
      fee: CLValueBuilder.u32(fee),
    });
    const deploy = this.contractClient.callEntrypoint(
      "set_acceptable_token",
      runtimeArgs,
      sender,
      this.networkName,
      paymentAmount,
      signingKeys
    );
    // if (signingKeys) {
    //   const deployHash = await this.casperClient.putDeploy(deploy);
    //   return deployHash;
    // }
    return deploy;
  }
}
