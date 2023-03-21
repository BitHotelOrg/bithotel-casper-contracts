use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum MarketplaceError {
    ContractAlreadyInitialized = 0,
    NFTInvalidHolderMode = 1,
    InvalidContext = 2,
    NFTRequireApprove = 3,
    NoListingOwner = 4,
    ListingNotFound = 5,
    ListingNotActive = 6,
    ListingOwnerCannotBuy = 7,
    ListingPriceIsZero = 8,
    CallerNotAdmin = 9,
    ContractNotWhitelisted = 10,
    ContractIsPaused = 11,
    ContractAlreadyPaused = 12,
    ContractAlreadyUnPaused = 13,
}

impl From<MarketplaceError> for ApiError {
    fn from(error: MarketplaceError) -> Self {
        ApiError::User(error as u16)
    }
}
