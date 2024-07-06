use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event{

    #[max_len(40)]
    pub name: String,
    pub ticket_price: u64,
    pub active: bool ,

    pub authority: PubKey,
    pub accepted_mint: PubKey,

    // this is the bumps of the PDAs

    pub event_bump: u8,
    pub event_mint_bump: u8,
    pub treasury_vault_: u8,

    

}