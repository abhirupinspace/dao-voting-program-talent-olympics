use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use crate::state::Proposal;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 3 * 8)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateProposal>, title: String, description: String) -> ProgramResult {
    let proposal = &mut ctx.accounts.proposal;
    proposal.title = title;
    proposal.description = description;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    Ok(())
}
