use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::token::{InitializeMint, MintTo, Token, Transfer};

declare_id!("GzcpjpMwSEYZKSi5ZupSfqTNaswBMxFhfESvXWbuURH1");

#[program]
pub mod token_contract {
    use anchor_lang::system_program;
    use anchor_spl::token::{burn, initialize_mint, mint_to, set_authority, Burn, SetAuthority};

    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, decimal: u8, amount: u64) -> Result<()> {
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.mint_token.to_account_info(),
                },
            ),
            10_000_000,
            82,
            ctx.accounts.token_program.key,
        )?;

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint_token.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            decimal,
            ctx.accounts.signer.key,
            Some(ctx.accounts.signer.key),
        )?;

     

        associated_token::create(CpiContext::new(
            ctx.accounts.associate_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.signer.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
                mint: ctx.accounts.mint_token.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(),
                MintTo {
                    authority: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.mint_token.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 10)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        anchor_spl::token::transfer(cpi_ctx, 5)?;
        Ok(())
    }

    pub fn set_authority_token(ctx: Context<SetAuthorityToken>) -> Result<()> {
        set_authority(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.mint_token.to_account_info(),
                    current_authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner,
            Some(ctx.accounts.new_signer.key()),
        )?;

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>, amount: u64) -> Result<()> {
        burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    authority: ctx.accounts.signer.to_account_info(),
                    from: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint_token.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct Metadata{
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub supply: u64,
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token: Signer<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub from: UncheckedAccount<'info>,

    #[account(mut)]
    pub to: UncheckedAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct SetAuthorityToken<'info> {
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub new_signer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
