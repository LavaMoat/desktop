use anyhow::{anyhow, Result};
use std::path::PathBuf;
use totp_rs::{Algorithm, TOTP};
use rand::thread_rng;
use eth_keystore::encrypt_key;
use ethers::signers::{coins_bip39::Wordlist, MnemonicBuilder, LocalWallet};

use crate::helpers::bip39::*;

/*
pub enum Screen {
    // Welcome screen
    Welcome,
    // BIP-39 12 word passphrase
    Passphrase(String),
    // BIP-39 24 word recovery seed
    RecoverySeed(String),
    // TOTP 2FA shared secret
    SeedTOTP {wallet: LocalWallet, url: String},
    // 2FA Verification
    Verify2FA,
    // Onboarding completed
    Complete,
}
*/

pub(crate) struct Totp {
    wallet: LocalWallet,
    url: String,
}

#[derive(Default)]
pub(crate) struct Onboarding<W> where W: Wordlist {
    passphrase: Option<String>,
    mnemonic: Option<String>,
    totp: Option<Totp>,
    marker: std::marker::PhantomData<W>,
}

impl<W> Onboarding<W> where W: Wordlist {
    pub(crate) fn new() -> Result<Self> {

        /*
        let passphrase = words(WordCount::short())?;
        let mnemonic = words(WordCount::long())?;

        // Random wallet for the TOTP secret
        let wallet = MnemonicBuilder::<English>::default()
            .build_random(&mut thread_rng())?;
        let private_key = wallet.signer().to_bytes().to_vec();
        let secret = hex::encode(&private_key);

        //let uuid = encrypt_key(&file, &mut rng, &private_key, passphrase)?;

        let totp = TOTP::new(
            Algorithm::SHA512,
            6,
            1,
            30,
            &secret,
        );
        let url = totp.get_url("", "metamask.io");
        */

        Ok(Self {
            passphrase: None,
            mnemonic: None,
            totp: None,
            marker: Default::default(),
        })
    }

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
        let wallet = MnemonicBuilder::<W>::default()
            .build_random(&mut thread_rng())?;

        let private_key = wallet.signer().to_bytes().to_vec();
        let secret = hex::encode(&private_key);
        let totp = TOTP::new(
            Algorithm::SHA512,
            6,
            1,
            30,
            &secret,
        );
        let url = totp.get_url("", "metamask.io");
        self.totp = Some(Totp{wallet, url});
        Ok(self)
    }

    // Complete the onboarding by writing the files to disc.
    pub fn build(self, file: &PathBuf) -> Result<String> {
        let passphrase = self.passphrase
            .ok_or_else(|| anyhow!("passphrase is not configured"))?;
        let mnemonic = self.mnemonic
            .ok_or_else(|| anyhow!("mnemonic is not configured"))?;
        let totp = self.totp
            .ok_or_else(|| anyhow!("totp is not configured"))?;

        let mut rng = thread_rng();
        let private_key = totp.wallet.signer().to_bytes().to_vec();

        // Write the TOTP private key to disc
        let uuid = encrypt_key(file, &mut rng, &private_key, passphrase)?;

        // TODO: write out the primary wallet to disc

        Ok(uuid)
    }
}
