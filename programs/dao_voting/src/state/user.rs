use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub points: u64,
}
