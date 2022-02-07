//! Encapsulates user private data and settings.
use std::path::PathBuf;
use std::sync::RwLock;
use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use ethers::prelude::*;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use eth_keystore::encrypt_key;

use crate::helpers::{format_address, bip39::words};

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccountView {
    address: Address,
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

    /// Get the application-specific storage directory.
    fn storage(&self) -> Result<PathBuf> {
        let base = home::home_dir()
            .ok_or_else(|| anyhow!("could not determine home directory"))?;

        // FIXME: OS-specific locations!
        let storage = base.join("Library").join("MetaMask");
        Ok(storage)
    }

    /// Determine if this user has an account on disc.
    pub fn exists(&self) -> Result<bool> {
        let file = self.storage()?.join(KEYSTORE);
        Ok(file.is_dir())
    }

    /// Create a new account for this user on disc.
    pub fn signup(&mut self, passphrase: &str) -> Result<Signup> {
        if self.exists()? {
            bail!("cannot signup with existing account");
        }

        // Create the master login keystore
        let file = self.storage()?.join(KEYSTORE);
        if !file.is_dir() {
            std::fs::create_dir_all(&file)?;
        }

        // Generate a mnemonic for the account seed recovery
        let mnemonic = words(Default::default())?;

        let wallet = MnemonicBuilder::<English>::default()
            .phrase(&mnemonic[..])
            .build()?;

        let private_key = wallet.signer().to_bytes().to_vec();
        let address = format_address(wallet.address());

        /*
        let address = format_address(wallet.address());
        let user_wallet = UserWallet::SingleParty(wallet);
        self.wallets.push(user_wallet);
        */

        // Store the keystore to disc
        let mut rng = rand::thread_rng();
        let uuid = encrypt_key(
            &file, &mut rng, &private_key, passphrase)?;

        let account = AccountView {
            address: address.clone()
        };
        self.user_data.accounts.insert(uuid, account);
        self.save()?;

        Ok(Signup {address, mnemonic})
    }

    /// Create the user's master seed key.
    pub fn create(&mut self, passphrase: &str) -> Result<()> {
        Ok(())
    }

    /// Login to the user's account.
    pub fn login(&mut self, passphrase: &str) -> Result<()> {
        //println!("Trying to decrypt masterseed...");
        //let wallet = Wallet::decrypt_keystore(path, passphrase)?;
        //println!("Wallet decryption finished!! {:?}", wallet);
        Ok(())
    }

    /// Create a new single party account.
    pub fn add_account(&mut self) -> Result<String> {
        todo!()
        /*
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_PHRASE)
            .build()?;
        let address = format_address(wallet.address());
        let user_wallet = UserWallet::SingleParty(wallet);
        self.wallets.push(user_wallet);
        Ok(address)
        */
    }

    /// List the user's accounts.
    pub fn list_accounts(&self) -> Vec<&AccountView> {
        self.user_data.accounts.values().collect()
    }
}

#[cfg(test)]
mod test {
    // https://chainid.network/chains.json

    //use ethers_core::rand::thread_rng;
    use ethers::prelude::*;
    use ethers::signers::{coins_bip39::English, MnemonicBuilder};

    #[test]
    fn create_wallet() -> anyhow::Result<()> {
        // Deterministic wallet generation from seed phrase
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_PHRASE)
            .build()?;
        println!("Wallet {:#?}", wallet);
        Ok(())
    }
}
