use anchor_lang::prelude::*;
use zoya_profile::state::DriverProfile;

use crate::errors::RideError;
use crate::state::{RideRequest, RideStatus};

#[derive(Accounts)]
pub struct AcceptRide<'info> {
    #[account(
        mut,
        constraint = ride_request.status == RideStatus::Pending @ RideError::InvalidRideStatus,
    )]
    pub ride_request: Account<'info, RideRequest>,

    /// Driver's profile — proves the signer is a registered driver.
    #[account(
        seeds = [b"driver_profile", driver.key().as_ref()],
        bump = driver_profile.bump,
        seeds::program = zoya_profile::ID,
    )]
    pub driver_profile: Account<'info, DriverProfile>,

    pub driver: Signer<'info>,
}

pub fn handler(ctx: Context<AcceptRide>) -> Result<()> {
    let ride = &mut ctx.accounts.ride_request;
    ride.driver = ctx.accounts.driver.key();
    ride.status = RideStatus::Accepted;
    ride.accepted_at = Clock::get()?.unix_timestamp;
    Ok(())
}
