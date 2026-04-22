use anchor_lang::prelude::*;

#[error_code]
pub enum RideError {
    #[msg("Pickup address is too long (max 128 chars)")]
    PickupTooLong,
    #[msg("Dropoff address is too long (max 128 chars)")]
    DropoffTooLong,
    #[msg("Fare must be greater than zero")]
    InvalidFare,
    #[msg("Ride is not in the expected status")]
    InvalidRideStatus,
    #[msg("Unauthorized — signer is not the ride driver")]
    UnauthorizedDriver,
    #[msg("Unauthorized — signer is not the ride rider")]
    UnauthorizedRider,
    #[msg("Cannot cancel a ride that is already in progress")]
    CannotCancelInProgress,
}
