use anyhow::{bail, Result};
use std::time::SystemTime;

use ethers::prelude::*;
use ethers::signers::coins_bip39::Wordlist;

use tinyfiledialogs::{password_box, input_box};

use super::{User, AccountView, AccountKind};

/// Perform the steps to authenticate a user.
///
/// * Prompt for the account passphrase.
/// * Decrypt the primary wallet using the supplied passphrase.
/// * Decrypt the TOTP secret using the supplied passphrase.
/// * Prompt for a TOTP 2FA token.
pub fn authenticate<W>(user: &mut User<W>) -> Result<Option<AccountView>> where W: Wordlist {
    if let Some(passphrase) =
        password_box("MetaMask", "Enter your account passphrase:")
    {
        // Use in-memory user data for login check
        let user_data = user.load_user_data()?;

        if let Some(user_data) = user_data {
            let primary = user_data
                .accounts
                .iter()
                .find(|(_, v)| v.kind == AccountKind::Primary);
            if let Some((uuid, account)) = primary {
                let primary_wallet = user.keystore()?.join(uuid);
                let _ = Wallet::decrypt_keystore(primary_wallet, &passphrase)?;

                if let Some(totp) = &user_data.totp {

                    let totp = user.storage()?.join(totp);
                    let secret = eth_keystore::decrypt_key(totp, &passphrase)?;
                    let totp = super::new_totp(&secret);
                    if let Some(token) =
                        input_box("MetaMask", "Enter your 2FA code:", "")
                    {
                        let time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)?
                            .as_secs();

                        if totp.check(&token, time) {
                            // Store the user data in-memory as they
                            // are now authenticated with 2FA verification
                            user.load()?;
                            Ok(Some(account.clone()))
                        } else {
                            bail!("invalid 2FA token");
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    // Store the user data in-memory as they
                    // are now authenticated but not using 2FA
                    user.load()?;
                    Ok(Some(account.clone()))
                }
            } else {
                bail!("cannot login without primary account");
            }
        } else {
            bail!("cannot login without user data");
        }
    } else {
        Ok(None)
    }
}
