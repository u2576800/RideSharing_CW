use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::state::DriverProfile;

#[derive(Accounts)]
pub struct RegisterDriver<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + DriverProfile::INIT_SPACE,
        seeds = [b"driver_profile", authority.key().as_ref()],
        bump
    )]
    pub driver_profile: Account<'info, DriverProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterDriver>,
    name: String,
    vehicle_info: String,
    phone: String,
) -> Result<()> {
    require!(name.len() <= 32, ProfileError::NameTooLong);
    require!(vehicle_info.len() <= 64, ProfileError::VehicleInfoTooLong);
    require!(phone.len() <= 16, ProfileError::PhoneTooLong);

    let profile = &mut ctx.accounts.driver_profile;
    profile.authority = ctx.accounts.authority.key();
    profile.name = name;
    profile.vehicle_info = vehicle_info;
    profile.phone = phone;
    profile.is_available = false;
    profile.total_rides_completed = 0;
    profile.rating = 500;
    profile.bump = ctx.bumps.driver_profile;
    profile.created_at = Clock::get()?.unix_timestamp;

    Ok(())
}
