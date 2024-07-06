use {create::Event, anchor_lang::prelude::*,anchor_spl::token::*};

#[derive(Accounts)]
pub struct CreateEvent<'info>{

    #[account(init, seeds = [Event::SEED_EVENT.as_ref(), authority.key().as_ref]),bump, payer =authority, mint::decimals = 0,
    mint::authority = event,
}