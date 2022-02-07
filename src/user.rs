//! Encapsulates user private data and settings.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use eth_keystore::encrypt_key;
use ethers::prelude::*;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};

use crate::helpers::{bip39::words, format_address};

const ACCOUNTS: &str = "accounts.json";
const KEYSTORE: &str = "keystore";

type UUID = String;
type Address = String;

pub static USER_DATA: Lazy<RwLock<User>> =
    Lazy::new(|| RwLock::new(Default::default()));

/// Load the user data.
pub fn load_user_data() -> Result<()> {
    let mut user = USER_DATA.write().unwrap();
    user.load()
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum AccountKind {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "imported")]
    Imported,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
pub struct Signup {
    address: Address,
    mnemonic: String,
}

#[derive(Debug, Default)]
pub struct User {
    user_data: UserData,
}

impl User {
    /// Save the user data to disc.
    fn save(&mut self) -> Result<()> {
        let file = self.storage()?.join(ACCOUNTS);
        let contents = serde_json::to_string_pretty(&self.user_data)?;
        std::fs::write(file, contents)?;
        Ok(())
    }

    /// Load the user data from disc.
    fn load(&mut self) -> Result<()> {
        let file = self.storage()?.join(ACCOUNTS);
        if file.exists() && file.is_file() {
            let contents = std::fs::read_to_string(file)?;
            let user_data: UserData = serde_json::from_str(&contents)?;
            self.user_data = user_data;
        }
        Ok(())
    }

    /// Find the primary account.
    fn primary(&self) -> Option<(&UUID, &AccountView)> {
        self.user_data
            .accounts
            .iter()
            .find(|(k, v)| v.kind == AccountKind::Primary)
    }

    /// Get the application-specific storage directory.
    fn storage(&self) -> Result<PathBuf> {
        let base = home::home_dir()
            .ok_or_else(|| anyhow!("could not determine home directory"))?;

        // FIXME: OS-specific locations!
        let storage = base.join("Library").join("MetaMask");
        Ok(storage)
    }

    /// Determine if this user has a primary account loaded.
    pub fn exists(&self) -> bool {
        self.primary().is_some()
    }

    /// Create a new account for this user on disc.
    pub fn signup(&mut self, passphrase: &str) -> Result<Signup> {
        if self.exists() {
            bail!("cannot signup with existing primary account");
        }

        // Create the keystore folder
        let file = self.storage()?.join(KEYSTORE);
        if !file.is_dir() {
            std::fs::create_dir_all(&file)?;
        }

        // Generate a mnemonic for the account seed recovery
        let mnemonic = words(Default::default())?;
        // Deterministic wallet from the seed recovery mnemonic
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(&mnemonic[..])
            .build()?;

        let private_key = wallet.signer().to_bytes().to_vec();
        let address = format_address(wallet.address());

        // Store the keystore to disc
        let mut rng = rand::thread_rng();
        let uuid = encrypt_key(&file, &mut rng, &private_key, passphrase)?;

        let account = AccountView {
            address: address.clone(),
            kind: AccountKind::Primary,
        };
        self.user_data.accounts.insert(uuid, account);
        self.save()?;

        Ok(Signup { address, mnemonic })
    }

    /// Create the user's master seed key.
    pub fn create(&mut self, passphrase: &str) -> Result<()> {
        Ok(())
    }

    /// Login to the user's account.
    ///
    /// Decrypts the primary keystore to verify the user can
    /// access the account.
    pub fn login(&mut self, passphrase: &str) -> Result<&AccountView> {
        if let Some((uuid, account)) = self.primary() {
            let path = self.storage()?.join(KEYSTORE).join(uuid);
            let _ = Wallet::decrypt_keystore(path, passphrase)?;
            Ok(account)
        } else {
            bail!("cannot login without primary account");
        }
    }

    /// List the user's accounts.
    pub fn list_accounts(&self) -> Vec<&AccountView> {
        self.user_data.accounts.values().collect()
    }

    /// Add a derived account.
    pub fn add_account(&mut self) -> Result<String> {
        todo!()
    }
}

// https://chainid.network/chains.json
