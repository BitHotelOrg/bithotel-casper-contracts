# Casper Marketplace Smart Contract
### [Audited by Halborn](https://github.com)

Most marketplace logic is defined in [contract/src/marketplace.rs](https://github.com/BitHotelOrg/bithotel-casper-contracts/blob/main/marketplace/contract/src/marketplace.rs), which consists of the main entry points: `add_listing`, `cancel_listing` and `execute_listing`.

A brief explanation of these entry points:

### `add_listing`
This entry point allows users to make a listing on our marketplace for their CEP78 token. This contract hash has to be whitelisted by the Bit Hotel team, because this contract only allows for a certain set of CEP78 contract hashes. Furthermore, the user has to provide the token id which they want to sell, and the price. Before calling this, the user has to approve the marketplace on it's CEP78 token. This is needed, because by calling this entry point, the marketplace will transfer the token from the user's purse to the marketplace's purse.

### `cancel_listing`
This entry point allows the user to cancel the listing of their CEP78 token. By providing the listing_id, the status of the listing will be changed to Canceled and the token will be send back to the user.

### `execute_listing`
This entry points allows a user to purchase a CEP78 token which in listed in the contract. This has to be called via the client contract, located at [`client/execute_listing_session`](https://github.com/BitHotelOrg/bithotel-casper-contracts/tree/main/marketplace/client/execute_listing_session). The user has to provide the amount in the runtime arguments to allow the session code to spend the Casper tokens. From here, the contract sets the status to Executed and the CEP78 token will be send to the buyer.