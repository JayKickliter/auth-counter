use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AuthCounter {
    pub authority: Pubkey,
    pub count: u32,
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    assert_eq!(std::mem::size_of::<AuthCounter>(), 36);

    if !authority.is_signer {
        msg!("Authority is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if account.owner != program_id {
        msg!("this program does not own account {:?}", account);
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ctr = AuthCounter::try_from_slice(&account.data.borrow())?;

    if ctr.count == 0 {
        ctr.authority = *authority.key
    } else if &ctr.authority != authority.key {
        msg!(
            "counter authority {:?} does not match provided authority {:?}",
            ctr.authority,
            account.key
        );
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    match instruction_data {
        &[] => ctr.count += 1,
        &[b0, b1, b2, b3] => ctr.count += u32::from_le_bytes([b0, b1, b2, b3]),
        _ => return Err(ProgramError::InvalidArgument),
    };

    ctr.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
