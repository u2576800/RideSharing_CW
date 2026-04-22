use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RiderProfile {
    pub authority: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(16)]
    pub phone: String,
    pub total_rides: u64,
    pub rating: u16,
    pub bump: u8,
    pub created_at: i64,
}
