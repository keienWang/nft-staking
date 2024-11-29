use super::StakingInstance;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InitializeStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        seeds = [crate::STAKING_SEED.as_ref(),authority.key().as_ref()],
        bump,
        space = 8 + core::mem::size_of::<StakingInstance>(),
        payer = authority
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    pub reward_token_mint: Account<'info, Mint>,
    pub staking_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}
