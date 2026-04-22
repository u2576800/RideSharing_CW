use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::state::RiderProfile;

#[derive(Accounts)]
pub struct IncrementRiderCount<'info> {
    #[account(
        mut,
        seeds = [b"rider_profile", rider_profile.authority.as_ref()],
        bump = rider_profile.bump,
    )]
    pub rider_profile: Account<'info, RiderProfile>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<IncrementRiderCount>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.rider_profile.authority,
        ctx.accounts.authority.key(),
        ProfileError::Unauthorized
    );

    let profile = &mut ctx.accounts.rider_profile;
    profile.total_rides = profile.total_rides.saturating_add(1);

    Ok(())
}
