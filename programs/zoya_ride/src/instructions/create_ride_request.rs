use anchor_lang::prelude::*;
use zoya_profile::cpi::accounts::IncrementRiderCount;
use zoya_profile::program::ZoyaProfile;
use zoya_profile::state::RiderProfile;

use crate::errors::RideError;
use crate::state::{RideRequest, RideStatus};

#[derive(Accounts)]
pub struct CreateRideRequest<'info> {
    #[account(
        init,
        payer = rider,
        space = 8 + RideRequest::INIT_SPACE,
        seeds = [
            b"ride_request",
            rider.key().as_ref(),
            &rider_profile.total_rides.to_le_bytes()
        ],
        bump
    )]
    pub ride_request: Account<'info, RideRequest>,

    #[account(
        mut,
        seeds = [b"rider_profile", rider.key().as_ref()],
        bump = rider_profile.bump,
        seeds::program = zoya_profile::ID,
    )]
    pub rider_profile: Account<'info, RiderProfile>,

    #[account(mut)]
    pub rider: Signer<'info>,

    pub zoya_profile_program: Program<'info, ZoyaProfile>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateRideRequest>,
    pickup_address: String,
    dropoff_address: String,
    fare_lamports: u64,
) -> Result<()> {
    require!(pickup_address.len() <= 128, RideError::PickupTooLong);
    require!(dropoff_address.len() <= 128, RideError::DropoffTooLong);
    require!(fare_lamports > 0, RideError::InvalidFare);

    let ride = &mut ctx.accounts.ride_request;
    ride.rider = ctx.accounts.rider.key();
    ride.driver = Pubkey::default();
    ride.pickup_address = pickup_address;
    ride.dropoff_address = dropoff_address;
    ride.fare_lamports = fare_lamports;
    ride.status = RideStatus::Pending;
    ride.ride_index = ctx.accounts.rider_profile.total_rides;
    ride.bump = ctx.bumps.ride_request;
    ride.created_at = Clock::get()?.unix_timestamp;
    ride.accepted_at = 0;
    ride.completed_at = 0;

    // CPI: increment rider's total_rides so the next ride uses a different PDA index
    let cpi_accounts = IncrementRiderCount {
        rider_profile: ctx.accounts.rider_profile.to_account_info(),
        authority: ctx.accounts.rider.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.zoya_profile_program.to_account_info(),
        cpi_accounts,
    );
    zoya_profile::cpi::increment_rider_count(cpi_ctx)?;

    Ok(())
}
