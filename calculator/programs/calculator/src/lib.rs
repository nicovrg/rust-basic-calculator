use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("535ydHRcLpUpsCqzHCs3Eg8DFWYKJpsk3qAVh9WqTf8n");

#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }

    pub fn add(ctx: Context<Addition>, nb1: i64, nb2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = nb1 + nb2;
        Ok(())
    }
    
    pub fn sub(ctx: Context<Substraction>, nb1: i64, nb2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = nb1 - nb2;
        Ok(())
    }

    pub fn mul(ctx: Context<Multiplication>, nb1: i64, nb2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = nb1 * nb2;
        Ok(())
    }

    pub fn div(ctx: Context<Division>, nb1: i64, nb2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        if nb2 != 0 {
            calculator.result = nb1 / nb2;
        } else {
            calculator.result = 0;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {

    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}


#[derive(Accounts)]
pub struct Substraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}


#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}


#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}



#[account]
pub struct Calculator {
    greeting: String,
    result: i64
}
