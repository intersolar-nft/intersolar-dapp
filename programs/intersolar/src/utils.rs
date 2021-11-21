use {
    crate::errors::ErrorCode,
    anchor_lang::{
        prelude::*,
        solana_program::program_pack::{IsInitialized, Pack},
    },
};

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}
