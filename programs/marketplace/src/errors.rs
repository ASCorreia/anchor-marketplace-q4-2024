use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("The String is too big")]
    StringLenghtInvalid
}