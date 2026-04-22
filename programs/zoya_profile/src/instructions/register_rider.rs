use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::state::RiderProfile;

#[derive(Accounts)]
pub struct RegisterRider<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RiderProfile::INIT_SPACE,
        seeds = [b"rider_profile", authority.key().as_ref()],
        bump
    )]
    pub rider_profile: Account<'info, RiderProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterRider>, name: String, phone: String) -> Result<()> {
    require!(name.len() <= 32, ProfileError::NameTooLong);
    require!(phone.len() <= 16, ProfileError::PhoneTooLong);

    let profile = &mut ctx.accounts.rider_profile;
    profile.authority = ctx.accounts.authority.key();
    profile.name = name;
    profile.phone = phone;
    profile.total_rides = 0;
    profile.rating = 500;
    profile.bump = ctx.bumps.rider_profile;
    profile.created_at = Clock::get()?.unix_timestamp;

    Ok(())
}
