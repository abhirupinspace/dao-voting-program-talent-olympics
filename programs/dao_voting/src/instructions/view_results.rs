use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use crate::state::Proposal;

#[derive(Accounts)]
pub struct ViewResults<'info> {
    pub proposal: Account<'info, Proposal>,
}

pub fn handler(ctx: Context<ViewResults>) -> ProgramResult {
    let proposal = &ctx.accounts.proposal;
    msg!("Title: {}", proposal.title);
    msg!("Description: {}", proposal.description);
    msg!("Yes Votes: {}", proposal.yes_votes);
    msg!("No Votes: {}", proposal.no_votes);
    msg!("Total Votes: {}", proposal.total_votes);
    Ok(())
}
