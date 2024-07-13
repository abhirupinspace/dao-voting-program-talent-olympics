use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User has already voted.")]
    AlreadyVoted,
    #[msg("Invalid proposal.")]
    InvalidProposal,
}


