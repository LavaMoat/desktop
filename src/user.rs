//! Encapsulates user private data and settings.
use ethers::signers::LocalWallet;

use std::sync::RwLock;
use once_cell::sync::Lazy;

pub static USER_DATA: Lazy<RwLock<User>> = Lazy::new(|| {
    RwLock::new(Default::default())
});

#[derive(Debug)]
pub enum UserWallet {
    Local(LocalWallet),
}

#[derive(Debug, Default)]
pub struct Account {
    wallets: Vec<UserWallet>,
}

#[derive(Debug, Default)]
pub struct User {
    accounts: Account,
}

#[cfg(test)]
mod test {
    // https://chainid.network/chains.json

    //use ethers_core::rand::thread_rng;
    use ethers::prelude::*;
    use ethers::signers::{MnemonicBuilder, coins_bip39::English};

    #[test]
    fn create_wallet() -> anyhow::Result<()> {
        // Deterministic wallet generation from seed phrase
        let wallet = MnemonicBuilder::<English>::default()
            .phrase("adult bachelor tower few mean enjoy fresh pull noise say basic gravity")
            .build()?;
        println!("Wallet {:#?}", wallet);
        Ok(())
    }
}
