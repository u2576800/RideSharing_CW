use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DriverProfile {
    pub authority: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(64)]
    pub vehicle_info: String,
    #[max_len(16)]
    pub phone: String,
    pub is_available: bool,
    pub total_rides_completed: u64,
    pub rating: u16,
    pub bump: u8,
    pub created_at: i64,
}
