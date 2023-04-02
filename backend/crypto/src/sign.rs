use rand_core::{CryptoRng, RngCore};
use rsa::pss::{BlindedSigningKey, Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, RandomizedSigner, Verifier};
use rsa::RsaPrivateKey;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("key error: {0}")]
    KeyError(#[from] rsa::errors::Error),

    #[error("signature error: {0}")]
    SignatureError(#[from] rsa::signature::Error),

    #[error("encoding error: {0}")]
    EncodingError(#[from] rmp_serde::encode::Error),

    #[error("decoding error: {0}")]
    DecodingError(String),
}

pub fn new_private_key<R>(rng: &mut R) -> Result<RsaPrivateKey, Error>
where
    R: CryptoRng + RngCore,
{
    let bits = 2048;
    Ok(RsaPrivateKey::new(rng, bits)?)
}

#[derive(Clone)]
pub struct EncodedPrivateKey(String);

impl EncodedPrivateKey {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for EncodedPrivateKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct Keys {
    pub signing_key: BlindedSigningKey<Sha256>,
    pub verifying_key: VerifyingKey<Sha256>,
}

impl Keys {
    pub fn generate<R>(rng: &mut R) -> Result<(RsaPrivateKey, Self), Error>
    where
        R: CryptoRng + RngCore,
    {
        let private_key = new_private_key(rng)?;
        Ok((private_key.clone(), Self::init(private_key)?))
    }

    pub fn init(private_key: RsaPrivateKey) -> Result<Self, Error> {
        let signing_key = new_signing_key(private_key)?;
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    pub fn from_encoded<S: AsRef<str>>(private_key: S) -> Result<Self, Error> {
        let private_key = decode_private_key(private_key.as_ref())?;
        Self::init(private_key)
    }

    pub fn sign<R>(&self, rng: &mut R, data: &[u8]) -> Signature
    where
        R: CryptoRng + RngCore,
    {
        self.signing_key.sign_with_rng(rng, data)
    }

    pub fn verify(&self, data: &[u8], signature: Signature) -> Result<(), Error> {
        Ok(self.verifying_key.verify(data, &signature)?)
    }
}

fn new_signing_key(private_key: RsaPrivateKey) -> Result<BlindedSigningKey<Sha256>, Error> {
    Ok(BlindedSigningKey::new(private_key))
}

pub fn encode_private_key(key: RsaPrivateKey) -> Result<EncodedPrivateKey, Error> {
    let key = rmp_serde::to_vec(&key)?;
    let key = crate::encode_base64(key);
    Ok(EncodedPrivateKey(key))
}

pub fn decode_private_key<T: AsRef<[u8]>>(key: T) -> Result<RsaPrivateKey, Error> {
    let key = crate::decode_base64(key).map_err(|e| Error::DecodingError(e.to_string()))?;
    rmp_serde::from_slice(&key).map_err(|e| Error::DecodingError(e.to_string()))
}

pub fn signature_from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Signature, Error> {
    Ok(Signature::try_from(bytes.as_ref())?)
}
