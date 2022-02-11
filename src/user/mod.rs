//! Encapsulates user private data and settings.

// https://chainid.network/chains.json
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use totp_rs::{Algorithm, TOTP};

use eth_keystore::encrypt_key;
use ethers::prelude::*;
use ethers::signers::{
    coins_bip39::{English, Wordlist},
    MnemonicBuilder,
};

use crate::helpers::{format_address};

mod account;
mod login;

use account::AccountBuilder;
use login::authenticate;

const ACCOUNTS: &str = "accounts.json";
const KEYSTORE: &str = "keystore";
const TOTP: &str = "totp";

type UUID = String;
type Address = String;

pub static USER_DATA: Lazy<RwLock<User<English>>> =
    Lazy::new(|| RwLock::new(Default::default()));

/// Helper function to create a TOTP.
pub(crate) fn new_totp<T: AsRef<[u8]>>(secret: T) -> TOTP<T> {
    TOTP::new(Algorithm::SHA1, 6, 1, 30, secret)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum AccountKind {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "imported")]
    Imported,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountView {
    address: Address,
    kind: AccountKind,
    // TODO: label, account type etc.
}

// Serialized user data stored on disc.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserData {
    /// Map account UUID to public address.
    accounts: HashMap<UUID, AccountView>,
    /// Relative path to the TOTP secrets file.
    totp: Option<PathBuf>,
}

pub struct User<W>
where
    W: Wordlist,
{
    /// User data is available after authentication.
    user_data: Option<UserData>,
    /// Account builder is available during the signup process.
    account_builder: Option<AccountBuilder<W>>,
}

impl Default for User<English> {
    fn default() -> Self {
        Self {
            user_data: None,
            account_builder: None,
        }
    }
}

impl<W> User<W>
where
    W: Wordlist,
{
    /// Save the user data to disc.
    fn save(&mut self) -> Result<()> {
        let file = self.storage()?.join(ACCOUNTS);
        let contents = serde_json::to_string_pretty(&self.user_data)?;
        std::fs::write(file, contents)?;
        Ok(())
    }

    /// Load the user data from disc.
    fn load_user_data(&mut self) -> Result<Option<UserData>> {
        let file = self.storage()?.join(ACCOUNTS);
        if file.exists() && file.is_file() {
            let contents = std::fs::read_to_string(file)?;
            let user_data: UserData = serde_json::from_str(&contents)?;
            return Ok(Some(user_data));
        }
        Ok(None)
    }

    /// Load the user data from disc.
    fn load(&mut self) -> Result<()> {
        self.user_data = self.load_user_data()?;
        Ok(())
    }

    /// Find the primary account.
    fn primary(&self) -> Result<Option<(&UUID, &AccountView)>> {
        let user_data = self
            .user_data
            .as_ref()
            .ok_or_else(|| anyhow!("not logged in"))?;

        Ok(user_data
            .accounts
            .iter()
            .find(|(k, v)| v.kind == AccountKind::Primary))
    }

    /// Get the application-specific storage directory.
    fn storage(&self) -> Result<PathBuf> {
        let base = home::home_dir()
            .ok_or_else(|| anyhow!("could not determine home directory"))?;

        // FIXME: OS-specific locations!
        let file = base.join("Library").join("MetaMask");
        if !file.is_dir() {
            std::fs::create_dir_all(&file)?;
        }

        Ok(file)
    }

    // Get the keystore folder.
    fn keystore(&self) -> Result<PathBuf> {
        let file = self.storage()?.join(KEYSTORE);
        if !file.is_dir() {
            std::fs::create_dir_all(&file)?;
        }
        Ok(file)
    }

    // Get the TOTP 2FA folder.
    fn totp(&self) -> Result<PathBuf> {
        let file = self.storage()?.join(TOTP);
        if !file.is_dir() {
            std::fs::create_dir_all(&file)?;
        }
        Ok(file)
    }

    /// Determine if this user has a primary account loaded.
    pub fn exists(&self) -> Result<bool> {
        self.primary().map(|o| o.is_some())
    }

    /// Initialize the account builder.
    pub fn signup_start(&mut self) -> Result<()> {
        if self.user_data.is_some() && self.exists()? {
            bail!("cannot signup with existing primary account");
        }

        self.account_builder = Some(AccountBuilder::<W>::new());
        Ok(())
    }

    /// Generate the authentication passphrase for the new account.
    pub fn signup_passphrase(&mut self) -> Result<&str> {
        let account_builder = self
            .account_builder
            .as_mut()
            .ok_or_else(|| anyhow!("account signup has not been started"))?;
        Ok(account_builder.passphrase()?)
    }

    /// Generate the seed recovery mnemonic for the new account.
    pub fn signup_mnemonic(&mut self) -> Result<&str> {
        let account_builder = self
            .account_builder
            .as_mut()
            .ok_or_else(|| anyhow!("account signup has not been started"))?;
        Ok(account_builder.mnemonic()?)
    }

    /// Generate the TOTP secret and URL for the new account.
    pub fn signup_totp(&mut self) -> Result<&str> {
        let account_builder = self
            .account_builder
            .as_mut()
            .ok_or_else(|| anyhow!("account signup has not been started"))?;
        Ok(account_builder.totp()?)
    }

    /// Verify the TOTP 2FA for account signup.
    pub fn signup_verify(&self, token: &str) -> Result<bool> {
        let account_builder = self
            .account_builder
            .as_ref()
            .ok_or_else(|| anyhow!("account signup has not been started"))?;
        Ok(account_builder.verify(token)?)
    }

    /// Complete the signup process by writing files to disc.
    pub fn signup_build(&mut self) -> Result<AccountView> {
        let keystore = self.keystore()?;
        let totp = self.totp()?;
        let account_builder = self
            .account_builder
            .as_mut()
            .ok_or_else(|| anyhow!("account signup has not been started"))?;
        let (address, uuid, totp_uuid) = account_builder.build(&keystore, &totp)?;
        // Write out the account information
        let account = AccountView {
            address,
            kind: AccountKind::Primary,
        };

        // Relative path to the TOTP secrets file
        let totp_file = totp.join(totp_uuid);
        let totp_file = totp_file.strip_prefix(self.storage()?)?;

        let mut user_data: UserData = Default::default();
        user_data.totp = Some(totp_file.to_path_buf());
        user_data.accounts.insert(uuid, account.clone());
        self.user_data = Some(user_data);
        self.save()?;

        Ok(account)
    }

    /// Finish signup, zeroizing in-memory signup data.
    ///
    /// This should be called if aborting a signup process or
    /// when the signup process is completed.
    ///
    /// Whilst this function is infallible we use the usual
    /// fallible signature for consistency.
    pub fn signup_finish(&mut self) -> Result<()> {
        if let Some(mut builder) = self.account_builder.take() {
            builder.zeroize();
        }
        self.account_builder = None;
        Ok(())
    }

    #[deprecated]
    /// Recover a private key from a seed phrase mnemonic.
    pub fn recover(
        &mut self,
        mnemonic: &str,
        passphrase: &str,
        is_primary: bool,
    ) -> Result<String> {
        let file = self.keystore()?;
        if is_primary && self.exists()? {
            bail!("cannot recover, primary account already exists");
        }

        let user_data = self
            .user_data
            .as_mut()
            .ok_or_else(|| anyhow!("not logged in"))?;

        // Deterministic wallet from the seed recovery mnemonic
        let wallet = MnemonicBuilder::<W>::default()
            .phrase(&mnemonic[..])
            .build()?;
        let address = format_address(wallet.address());

        // Store the keystore to disc
        let mut rng = rand::thread_rng();
        let private_key = wallet.signer().to_bytes().to_vec();
        let uuid = encrypt_key(&file, &mut rng, &private_key, passphrase)?;

        let kind = if is_primary {
            AccountKind::Primary
        } else {
            AccountKind::Imported
        };

        let account = AccountView {
            address: address.clone(),
            kind,
        };
        user_data.accounts.insert(uuid, account);
        self.save()?;

        Ok(address)
    }

    /// Create the user's master seed key.
    pub fn create(&mut self, passphrase: &str) -> Result<()> {
        Ok(())
    }

    /// Login to the user's account.
    pub fn login(&mut self) -> Result<Option<AccountView>> {
        authenticate(self)
    }

    /// Logout of the account.
    pub fn logout(&mut self) -> Result<()> {
        self.user_data = None;
        Ok(())
    }

    /// List the user's accounts.
    pub fn list_accounts(&self) -> Result<Vec<&AccountView>> {
        let user_data = self
            .user_data
            .as_ref()
            .ok_or_else(|| anyhow!("not logged in"))?;
        Ok(user_data.accounts.values().collect())
    }

    /// Add a derived account.
    pub fn add_account(&mut self) -> Result<String> {
        todo!()
    }
}
