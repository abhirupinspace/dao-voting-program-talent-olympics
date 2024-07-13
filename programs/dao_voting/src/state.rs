use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
}

#[account]
pub struct User {
    pub has_voted: bool,
    pub points: u64,
}
