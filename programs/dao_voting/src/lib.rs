// use anchor_lang::prelude::*;

// pub mod errors;
// pub mod instructions;
// pub mod state;

// declare_id!("Fg6PaFpoGXkYsidMpWxTW8h3QhKBFakWX4i5hPUQZ6cA");

// pub mod dao_voting {
//     use anchor_lang::solana_program::entrypoint::ProgramResult;
//     use instructions::{create_proposal::CreateProposal, view_results::ViewResults, vote::Vote};

//     use super::*;

//     pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> ProgramResult {
//         instructions::create_proposal::handler(ctx, title, description)
//     }

//     // pub fn vote(ctx: Context<'_, '_, '_, '_, Vote>, support: bool) -> ProgramResult {
//     //     instructions::vote::handler(ctx, support)
//     // }

//     pub fn vote<'a, 'b, 'c, 'info>(
//         ctx: Context<'a, 'b, 'c, 'info, Vote>,
//         support: bool
//     ) -> ProgramResult {
//         instructions::vote::handler(ctx, support)
//     }
    

//     pub fn view_results(ctx: Context<ViewResults>) -> ProgramResult {
//         instructions::view_results::handler(ctx)
//     }
// }


use anchor_lang::prelude::*;
use instructions::{create_proposal::CreateProposal, view_results::ViewResults, vote::Vote};

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWxTW8h3QhKBFakWX4i5hPUQZ6cA");

pub mod dao_voting {
    use super::*;
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> ProgramResult {
        instructions::create_proposal::handler(ctx, title, description)
    }

    pub fn vote<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Vote<'info>>,
        support: bool
    ) -> ProgramResult {
        instructions::vote::handler(ctx, support)
    }

    pub fn view_results(ctx: Context<ViewResults>) -> ProgramResult {
        instructions::view_results::handler(ctx)
    }
}
