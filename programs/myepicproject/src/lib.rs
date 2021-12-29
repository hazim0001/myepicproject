// use anchor_lang::prelude::*;

// declare_id!("6PxF9jrJ7heTWw5B9GK9gHcJjqRMxiHp5XfrmBDLA4qm");

// #[program]
// pub mod myepicproject {
//     use super::*;
//     pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}
use anchor_lang::prelude::*;

declare_id!("6PxF9jrJ7heTWw5B9GK9gHcJjqRMxiHp5XfrmBDLA4qm");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {

  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,

  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
