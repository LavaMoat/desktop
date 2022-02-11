//! Account builder for creating a new account.
use anyhow::{anyhow, bail, Result};
use eth_keystore::encrypt_key;
use ethers::prelude::*;
use ethers::signers::{coins_bip39::Wordlist, LocalWallet, MnemonicBuilder};
use rand::thread_rng;
use rand::Rng;
use std::path::PathBuf;
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};
use zeroize::Zeroize;

use crate::helpers::{bip39::*, format_address};

#[derive(Zeroize)]
pub(crate) struct Totp {
    url: String,
    secret: String,
}

#[derive(Zeroize)]
pub(crate) struct AccountBuilder<W>
where
    W: Wordlist,
{
    pub(super) passphrase: Option<String>,
    pub(super) mnemonic: Option<String>,
    pub(super) totp: Option<Totp>,
    done: bool,
    #[zeroize(skip)]
    marker: std::marker::PhantomData<W>,
}

impl<W> AccountBuilder<W>
where
    W: Wordlist,
{
    pub fn new() -> Self {
        Self {
            passphrase: None,
            mnemonic: None,
            totp: None,
            done: false,
            marker: std::marker::PhantomData,
        }
    }

    /// Generate the login passphrase.
    pub fn passphrase(&mut self) -> Result<&str> {
        let passphrase = words(WordCount::short())?;
        self.passphrase = Some(passphrase);
        Ok(self.passphrase.as_ref().unwrap())
    }

    /// Generate the recovery seed mnemonic.
    pub fn mnemonic(&mut self) -> Result<&str> {
        let mnemonic = words(WordCount::long())?;
        self.mnemonic = Some(mnemonic);
        Ok(self.mnemonic.as_ref().unwrap())
    }

    /// Generate the TOTP secret and URL.
    ///
    /// We use a local wallet so that we can encrypt the
    /// TOTP secret using the login passphrase to protect
    /// the secret on disc.
    pub fn totp(&mut self) -> Result<&str> {
        // 256 bits of entropy for the TOTP secret
        let secret_bytes = thread_rng().gen::<[u8; 32]>();
        let secret = hex::encode(&secret_bytes);
        let totp = Self::new_totp(&secret);
        let url = totp.get_url("metamask", "metamask.io");
        self.totp = Some(Totp { secret, url });
        Ok(&self.totp.as_ref().unwrap().url)
    }

    /// Verify a TOTP token.
    pub fn verify(&self, token: &str) -> Result<bool> {
        let data = self
            .totp
            .as_ref()
            .ok_or_else(|| anyhow!("cannot verify, totp not set yet"))?;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let totp = Self::new_totp(&data.secret);
        Ok(totp.check(token, time))
    }

    fn new_totp<T: AsRef<[u8]>>(secret: T) -> TOTP<T> {
        TOTP::new(Algorithm::SHA1, 6, 1, 30, secret)
    }

    fn write_totp_wallet(
        &self,
        keystore_dir: &PathBuf,
        passphrase: &str,
        totp: &Totp,
    ) -> Result<String> {
        // Write the TOTP private key to disc
        let uuid = encrypt_key(
            keystore_dir,
            &mut thread_rng(),
            &totp.secret,
            passphrase,
        )?;
        Ok(uuid)
    }

    fn write_primary_wallet(
        &self,
        keystore_dir: &PathBuf,
        passphrase: &str,
        mnemonic: &str,
    ) -> Result<(String, String)> {
        // Deterministic wallet from the seed recovery mnemonic
        let wallet = Self::build_wallet(mnemonic)?;
        let address = format_address(wallet.address());

        // Store the keystore to disc
        let mut rng = thread_rng();
        let private_key = wallet.signer().to_bytes().to_vec();
        let uuid =
            encrypt_key(keystore_dir, &mut rng, &private_key, passphrase)?;
        Ok((address, uuid))
    }

    /// Deterministic wallet from a seed recovery mnemonic.
    pub fn build_wallet(mnemonic: &str) -> Result<LocalWallet> {
        let wallet =
            MnemonicBuilder::<W>::default().phrase(mnemonic).build()?;
        Ok(wallet)
    }

    // Create a new account by writing the files to disc.
    pub fn build(&mut self,
        keystore_dir: &PathBuf,
        totp_dir: &PathBuf) -> Result<(String, String, String)> {

        if self.done {
            bail!("account creation is done");
        }

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
            self.write_totp_wallet(totp_dir, passphrase, totp)?;
        let (address, uuid) =
            self.write_primary_wallet(keystore_dir, passphrase, mnemonic)?;

        self.done = true;

        Ok((
            address,
            uuid,
            totp_uuid,
        ))
    }
}
