use super::StakingInstance;
use super::User;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct CancelStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub staking_instance: Account<'info, StakingInstance>,
    #[account(mut)]
    pub user_instance: Account<'info, User>,
    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub gdtc_lp_in_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_gdtc_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub gdtc_reward_out_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
