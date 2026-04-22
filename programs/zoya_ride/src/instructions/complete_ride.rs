use anchor_lang::prelude::*;
use zoya_profile::cpi::accounts::IncrementRideCount;
use zoya_profile::program::ZoyaProfile;
use zoya_profile::state::DriverProfile;

use crate::errors::RideError;
use crate::state::{RideRequest, RideStatus};

#[derive(Accounts)]
pub struct CompleteRide<'info> {
    #[account(
        mut,
        constraint = ride_request.status == RideStatus::InProgress @ RideError::InvalidRideStatus,
        constraint = ride_request.driver == driver.key() @ RideError::UnauthorizedDriver,
    )]
    pub ride_request: Account<'info, RideRequest>,

    /// Driver's profile account (owned by zoya_profile). Mutated via CPI.
    #[account(
        mut,
        seeds = [b"driver_profile", driver.key().as_ref()],
        bump = driver_profile.bump,
        seeds::program = zoya_profile::ID,
    )]
    pub driver_profile: Account<'info, DriverProfile>,

    pub driver: Signer<'info>,

    pub zoya_profile_program: Program<'info, ZoyaProfile>,
}

pub fn handler(ctx: Context<CompleteRide>) -> Result<()> {
    let ride = &mut ctx.accounts.ride_request;
    ride.status = RideStatus::Completed;
    ride.completed_at = Clock::get()?.unix_timestamp;

    let cpi_accounts = IncrementRideCount {
        driver_profile: ctx.accounts.driver_profile.to_account_info(),
        authority: ctx.accounts.driver.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.zoya_profile_program.to_account_info(),
        cpi_accounts,
    );
    zoya_profile::cpi::increment_ride_count(cpi_ctx)?;

    Ok(())
}
