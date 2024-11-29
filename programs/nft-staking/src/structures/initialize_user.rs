use super::{StakingInstance, User};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]

pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        space = 8 + core::mem::size_of::<User>(),
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        payer = authority,
    )]
    pub user_instance: Box<Account<'info, User>>,
    #[account(mut)]
    pub staking_instance: Account<'info, StakingInstance>,
    #[account(mut)]
    pub user_Superior_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
