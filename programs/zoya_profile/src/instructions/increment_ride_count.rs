use anchor_lang::prelude::*;

use crate::errors::ProfileError;
use crate::state::DriverProfile;

#[derive(Accounts)]
pub struct IncrementRideCount<'info> {
    #[account(
        mut,
        seeds = [b"driver_profile", driver_profile.authority.as_ref()],
        bump = driver_profile.bump,
    )]
    pub driver_profile: Account<'info, DriverProfile>,

    /// The driver must sign. This prevents arbitrary programs from bumping
    /// the counter without the driver's authorization.
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<IncrementRideCount>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.driver_profile.authority,
        ctx.accounts.authority.key(),
        ProfileError::Unauthorized
    );

    let profile = &mut ctx.accounts.driver_profile;
    profile.total_rides_completed = profile.total_rides_completed.saturating_add(1);

    Ok(())
}
