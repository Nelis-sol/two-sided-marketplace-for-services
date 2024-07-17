use anchor_lang::prelude::*;
use std::str::FromStr;

pub fn get_accountinfo_option(account_option: Option<AccountInfo>) -> Option<AccountInfo> {
    if let Some(account) = account_option {
        if *account.key != Pubkey::from_str("xxxxAbEHHieDpokcQLYLQUgzgU2f92ERnd8w3McEpf8").unwrap() {
            // If the account key is not the default, return Some(key)
            return Some(account);
        }
    }
    // Return None if the option is None or the key is the default
    None
}

pub fn get_signer_option(account_option: Option<Signer>) -> Option<Signer> {
    if let Some(account) = account_option {
        if *account.key != Pubkey::from_str("xxxxAbEHHieDpokcQLYLQUgzgU2f92ERnd8w3McEpf8").unwrap() {
            // If the account key is not the default, return Some(key)
            return Some(account);
        }
    }
    // Return None if the option is None or the key is the default
    None
}