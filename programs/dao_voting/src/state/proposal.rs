use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
}
