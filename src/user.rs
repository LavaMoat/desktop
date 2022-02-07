//! Encapsulates user private data and settings.
use anyhow::Result;
//use ethers::signers::LocalWallet;

use ethers::prelude::*;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

const TEST_PHRASE: &str =
    "adult bachelor tower few mean enjoy fresh pull noise say basic gravity";

fn format_address(address: H160) -> String {
    format!("0x{}", hex::encode(address.0))
}

pub static USER_DATA: Lazy<RwLock<User>> =
    Lazy::new(|| RwLock::new(Default::default()));

#[derive(Debug)]
pub enum UserWallet {
    SingleParty(LocalWallet),
}

#[derive(Serialize, Deserialize)]
pub struct AccountView {
    address: String,
}

impl From<&UserWallet> for AccountView {
    fn from(value: &UserWallet) -> Self {
        match value {
            UserWallet::SingleParty(wallet) => Self {
                address: format_address(wallet.address()),
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct User {
    wallets: Vec<UserWallet>,
}

impl User {
    /// Create a new single party account.
    pub fn create_account(&mut self) -> Result<String> {
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(TEST_PHRASE)
            .build()?;
        let address = format_address(wallet.address());
        let user_wallet = UserWallet::SingleParty(wallet);
        self.wallets.push(user_wallet);
        Ok(address)
    }

    /// List the user's accounts.
    pub fn list_accounts(&self) -> Vec<AccountView> {
        self.wallets.iter().map(|w| w.into()).collect()
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
