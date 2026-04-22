use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum RideStatus {
    Pending,
    Accepted,
    InProgress,
    Completed,
    Cancelled,
}

#[account]
#[derive(InitSpace)]
pub struct RideRequest {
    pub rider: Pubkey,
    pub driver: Pubkey,
    #[max_len(128)]
    pub pickup_address: String,
    #[max_len(128)]
    pub dropoff_address: String,
    pub fare_lamports: u64,
    pub status: RideStatus,
    pub ride_index: u64,
    pub bump: u8,
    pub created_at: i64,
    pub accepted_at: i64,
    pub completed_at: i64,
}
