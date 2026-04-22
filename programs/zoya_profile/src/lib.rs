use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("6h5cXVWNVtZ45XXQ5k1mHFQXtyeaUuYB7mUgvHc8kaCV");

#[program]
pub mod zoya_profile {
    use super::*;

    pub fn register_driver(
        ctx: Context<RegisterDriver>,
        name: String,
        vehicle_info: String,
        phone: String,
    ) -> Result<()> {
        instructions::register_driver::handler(ctx, name, vehicle_info, phone)
    }

    pub fn register_rider(
        ctx: Context<RegisterRider>,
        name: String,
        phone: String,
    ) -> Result<()> {
        instructions::register_rider::handler(ctx, name, phone)
    }

    pub fn increment_ride_count(ctx: Context<IncrementRideCount>) -> Result<()> {
        instructions::increment_ride_count::handler(ctx)
    }

    pub fn increment_rider_count(ctx: Context<IncrementRiderCount>) -> Result<()> {
        instructions::increment_rider_count::handler(ctx)
    }
}
