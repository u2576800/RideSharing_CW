use anchor_lang::prelude::*;

#[error_code]
pub enum ProfileError {
    #[msg("Name is too long (max 32 chars)")]
    NameTooLong,
    #[msg("Vehicle info is too long (max 64 chars)")]
    VehicleInfoTooLong,
    #[msg("Phone is too long (max 16 chars)")]
    PhoneTooLong,
    #[msg("Unauthorized — signer does not own this profile")]
    Unauthorized,
}
