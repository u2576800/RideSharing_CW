use anchor_lang::prelude::*;

use crate::errors::RideError;
use crate::state::{RideRequest, RideStatus};

#[derive(Accounts)]
pub struct CancelRide<'info> {
    #[account(
        mut,
        constraint = matches!(
            ride_request.status,
            RideStatus::Pending | RideStatus::Accepted
        ) @ RideError::CannotCancelInProgress,
        constraint = (
            ride_request.rider == signer.key()
            || ride_request.driver == signer.key()
        ) @ RideError::UnauthorizedRider,
    )]
    pub ride_request: Account<'info, RideRequest>,

    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<CancelRide>) -> Result<()> {
    ctx.accounts.ride_request.status = RideStatus::Cancelled;
    Ok(())
}
