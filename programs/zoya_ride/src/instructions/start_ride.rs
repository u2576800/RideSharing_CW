use anchor_lang::prelude::*;

use crate::errors::RideError;
use crate::state::{RideRequest, RideStatus};

#[derive(Accounts)]
pub struct StartRide<'info> {
    #[account(
        mut,
        constraint = ride_request.status == RideStatus::Accepted @ RideError::InvalidRideStatus,
        constraint = ride_request.driver == driver.key() @ RideError::UnauthorizedDriver,
    )]
    pub ride_request: Account<'info, RideRequest>,

    pub driver: Signer<'info>,
}

pub fn handler(ctx: Context<StartRide>) -> Result<()> {
    ctx.accounts.ride_request.status = RideStatus::InProgress;
    Ok(())
}
