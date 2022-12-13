use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum MarketplaceError {
    NFTInvalidHolderMode = 1,
    InvalidContext = 2,
    NFTRequireApprove = 3,
    NoListingOwner = 4,
    ListingNotFound = 5,
    ListingNotActive = 6,
}

impl From<MarketplaceError> for ApiError {
    fn from(error: MarketplaceError) -> Self {
        ApiError::User(error as u16)
    }
}
