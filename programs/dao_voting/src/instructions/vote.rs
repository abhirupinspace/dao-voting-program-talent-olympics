use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use crate::state::{Proposal, User};
use crate::errors::ErrorCode; // Adjust this import based on your project structure

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Account<'info, User>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Vote<'info>>, support: bool) -> ProgramResult {
    let proposal = &mut ctx.accounts.proposal;
    let user = &mut ctx.accounts.user;

    // Ensure the user has not already voted
    assert!(!user.has_voted, "{}", ErrorCode::AlreadyVoted as u32);

    // Update proposal votes based on support
    if support {
        proposal.yes_votes += 1;
    } else {
        proposal.no_votes += 1;
    }

    proposal.total_votes += 1;

    // Mark user as voted and reward points
    user.has_voted = true;
    user.points += 1;  // Reward points for participation

    Ok(())
}
