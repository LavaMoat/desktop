use anyhow::{anyhow, Result};
use eth_keystore::encrypt_key;
use ethers::prelude::*;
use ethers::signers::{coins_bip39::Wordlist, LocalWallet, MnemonicBuilder};
use rand::thread_rng;
use std::path::PathBuf;
use totp_rs::{Algorithm, TOTP};
use zeroize::ZeroizeOnDrop;

use crate::helpers::{bip39::*, format_address};

#[derive(ZeroizeOnDrop)]
pub(crate) struct Totp {
    url: String,
    #[zeroize(skip)]
    wallet: LocalWallet,
}

#[derive(Default, ZeroizeOnDrop)]
pub(crate) struct AccountBuilder<W>
where
    W: Wordlist,
{
    passphrase: Option<String>,
    mnemonic: Option<String>,
    totp: Option<Totp>,
    #[zeroize(skip)]
    marker: std::marker::PhantomData<W>,
}

impl<W> AccountBuilder<W>
where
    W: Wordlist,
{
    /// Generate the login passphrase.
    pub fn passphrase(mut self) -> Result<Self> {
        let passphrase = words(WordCount::short())?;
        self.passphrase = Some(passphrase);
        Ok(self)
    }

    /// Generate the recovery seed mnemonic.
    pub fn mnemonic(mut self) -> Result<Self> {
        let mnemonic = words(WordCount::long())?;
        self.mnemonic = Some(mnemonic);
        Ok(self)
    }

    /// Generate the TOTP secret and URL.
    ///
    /// We use a local wallet so that we can encrypt the
    /// TOTP secret using the login passphrase to protect
    /// the secret on disc.
    pub fn totp(mut self) -> Result<Self> {
        let wallet =
            MnemonicBuilder::<W>::default().build_random(&mut thread_rng())?;

        let private_key = wallet.signer().to_bytes().to_vec();
        let secret = hex::encode(&private_key);
        let totp = TOTP::new(Algorithm::SHA512, 6, 1, 30, &secret);
        let url = totp.get_url("", "metamask.io");
        self.totp = Some(Totp { wallet, url });
        Ok(self)
    }

    fn write_totp_wallet(
        &self,
        keystore_dir: &PathBuf,
        passphrase: &str,
        totp: &Totp,
    ) -> Result<String> {
        let mut rng = thread_rng();
        let private_key = totp.wallet.signer().to_bytes().to_vec();

        // Write the TOTP private key to disc
        let uuid =
            encrypt_key(keystore_dir, &mut rng, &private_key, passphrase)?;

        Ok(uuid)
    }

    fn write_primary_wallet(
        &self,
        keystore_dir: &PathBuf,
        passphrase: &str,
        mnemonic: &str,
    ) -> Result<(String, String)> {
        // Deterministic wallet from the seed recovery mnemonic
        let wallet =
            MnemonicBuilder::<W>::default().phrase(mnemonic).build()?;
        let address = format_address(wallet.address());

        // Store the keystore to disc
        let mut rng = thread_rng();
        let private_key = wallet.signer().to_bytes().to_vec();
        let uuid =
            encrypt_key(keystore_dir, &mut rng, &private_key, passphrase)?;
        Ok((address, uuid))
    }

    // Create a new account by writing the files to disc.
    pub fn build(&self, keystore_dir: &PathBuf) -> Result<String> {
        let passphrase = self
            .passphrase
            .as_ref()
            .ok_or_else(|| anyhow!("passphrase is not configured"))?;
        let mnemonic = self
            .mnemonic
            .as_ref()
            .ok_or_else(|| anyhow!("mnemonic is not configured"))?;
        let totp = self
            .totp
            .as_ref()
            .ok_or_else(|| anyhow!("totp is not configured"))?;

        let totp_uuid =
            self.write_totp_wallet(keystore_dir, passphrase, totp)?;

        let (address, uuid) =
            self.write_primary_wallet(keystore_dir, passphrase, mnemonic)?;

        Ok(totp_uuid)
    }
}
