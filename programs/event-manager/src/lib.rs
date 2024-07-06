use anchor_lang::prelude::*;

declare_id!("4jne4voX6MxJHP4Q2eUBHZ6uNc8WjzynnuhFXrREBEGd");

#[program]
pub mod event_manager {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
