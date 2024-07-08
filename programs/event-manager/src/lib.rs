use {anchor_lang::prelude::*,crate::instructions::*};

mod collections;
mod instructions;

declare_id!("4jne4voX6MxJHP4Q2eUBHZ6uNc8WjzynnuhFXrREBEGd");

#[program]
pub mod event_manager {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        ticket_price: u64,

    ) -> Result<()> {

instructions::create_event::handle(ctx, name, ticket_price)
    }

    
}