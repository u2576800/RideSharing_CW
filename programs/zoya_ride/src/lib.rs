use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("GuiYvQDwxG3VfufPekrhvBSWbnhN1a9ReVE4U9Bwp5Ro");

#[program]
pub mod zoya_ride {
    use super::*;

    pub fn create_ride_request(
        ctx: Context<CreateRideRequest>,
        pickup_address: String,
        dropoff_address: String,
        fare_lamports: u64,
    ) -> Result<()> {
        instructions::create_ride_request::handler(
            ctx,
            pickup_address,
            dropoff_address,
            fare_lamports,
        )
    }

    pub fn accept_ride(ctx: Context<AcceptRide>) -> Result<()> {
        instructions::accept_ride::handler(ctx)
    }

    pub fn start_ride(ctx: Context<StartRide>) -> Result<()> {
        instructions::start_ride::handler(ctx)
    }

    pub fn complete_ride(ctx: Context<CompleteRide>) -> Result<()> {
        instructions::complete_ride::handler(ctx)
    }

    pub fn cancel_ride(ctx: Context<CancelRide>) -> Result<()> {
        instructions::cancel_ride::handler(ctx)
    }
}
