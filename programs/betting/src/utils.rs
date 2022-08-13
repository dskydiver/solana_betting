use std::slice::Windows;

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
};

use getrandom;

pub static BATTLE_SEED: &[u8] = b"battle";
pub static ESCROW_SEED: &[u8] = b"escrow";
pub static BET_SEED: &[u8] = b"bet";
pub const ORDER_ESCROW_NATIVE_SIZE: usize = 0;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Winner {
    Left,
    Right,
}

// wrapper of 'create_account' instruction from 'system_program' program
#[inline(always)]
pub fn sys_create_account<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
    space: usize,
    owner: &Pubkey,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    invoke_signed(
        &system_instruction::create_account(from.key, to.key, lamports, space as u64, owner),
        &[from.clone(), to.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

#[inline(always)]
pub fn sys_transfer_unchecked<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
) -> Result<()> {
    invoke(
        &system_instruction::transfer(from.key, to.key, lamports),
        &[from.clone(), to.clone()],
    )?;

    Ok(())
}

#[inline(always)]
pub fn choose(left: u64, right: u64, seed: u64) -> Winner {
    let a : u64 = 1103515245;
    let c : u64 = 12345;
    let m : u64 = 2 ^ 32;
    let x = (a * seed + c) % m;

    if (left + right) * x / m < left {
        Winner::Left
    } else {
        Winner::Right
    }
}
