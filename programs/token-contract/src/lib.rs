use anchor_lang::prelude::*;
use anchor_spl::token; 
use anchor_spl::token::{Token, InitializeMint, MintTo, Transfer};

declare_id!("GzcpjpMwSEYZKSi5ZupSfqTNaswBMxFhfESvXWbuURH1");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint_token(ctx:Context<MintToken>, ) -> Result<()> {

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 10) ?;
        Ok(())
    }

    pub fn transfer_token(ctx:Context<TransferToken>) -> Result<()> {

        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction); 

        anchor_spl::token::transfer(cpi_ctx, 5) ?;
    Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct MintToken<'info > {

    #[account(mut)]
    pub mint: UncheckedAccount<'info >,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub token_account: UncheckedAccount<'info >,

    #[account(mut)]
    pub payer: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>, //

    #[account(mut)]
    pub from: UncheckedAccount<'info >,

    #[account(mut)]
    pub to: UncheckedAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info >
}