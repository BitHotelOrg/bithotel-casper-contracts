export function getEnvironmentVars(): {
  rpcUri: string;
  privateKey: string;
  nft: string;
  marketplace: string;
  chainName: string;
} {
  const env = String(process.env.NODE_ENV);
  const vars = {
    rpcUri: "",
    privateKey: "",
    nft: "",
    marketplace: "",
    chainName: "",
  };
  const suffix = env.toUpperCase();
  if (process.env.PRIVATE_KEY) {
    vars.privateKey = process.env.PRIVATE_KEY;
  }
  let rpcUri = process.env[`RPC_URI_${suffix}`];
  if (rpcUri) {
    vars.rpcUri = rpcUri;
  }
  const nftHash = process.env[`NFT_HASH_${suffix}`];
  if (nftHash) {
    vars.nft = nftHash;
  }
  const marketplaceHash = process.env[`MARKETPLACE_HASH_${suffix}`];
  if (marketplaceHash) {
    vars.marketplace = marketplaceHash;
  }
  vars.chainName = suffix == "PROD" ? "casper" : "casper-test";
  return vars;
}
