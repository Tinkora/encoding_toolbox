use std::str::FromStr;

use hmac::{Hmac, Mac as _};
use sha2::{Digest as _, Sha256, Sha384, Sha512};

use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigestAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Blake3,
    Md5,
}

impl DigestAlgorithm {
    pub const ALL: &[Self] = &[
        Self::Sha256,
        Self::Sha384,
        Self::Sha512,
        Self::Blake3,
        Self::Md5,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha384 => "sha384",
            Self::Sha512 => "sha512",
            Self::Blake3 => "blake3",
            Self::Md5 => "md5",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Sha256 => "SHA-256",
            Self::Sha384 => "SHA-384",
            Self::Sha512 => "SHA-512",
            Self::Blake3 => "BLAKE3",
            Self::Md5 => "MD5",
        }
    }

    pub const fn is_legacy(self) -> bool {
        matches!(self, Self::Md5)
    }
}

impl FromStr for DigestAlgorithm {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "sha256" | "sha-256" => Ok(Self::Sha256),
            "sha384" | "sha-384" => Ok(Self::Sha384),
            "sha512" | "sha-512" => Ok(Self::Sha512),
            "blake3" => Ok(Self::Blake3),
            "md5" => Ok(Self::Md5),
            _ => Err(Error::UnsupportedAlgorithm(value.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HmacAlgorithm {
    Sha256,
    Sha512,
}

impl HmacAlgorithm {
    pub const ALL: &[Self] = &[Self::Sha256, Self::Sha512];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Sha256 => "HMAC-SHA-256",
            Self::Sha512 => "HMAC-SHA-512",
        }
    }
}

impl FromStr for HmacAlgorithm {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "sha256" | "sha-256" | "hmac-sha256" | "hmac-sha-256" => Ok(Self::Sha256),
            "sha512" | "sha-512" | "hmac-sha512" | "hmac-sha-512" => Ok(Self::Sha512),
            _ => Err(Error::UnsupportedAlgorithm(value.to_owned())),
        }
    }
}

pub fn digest(algorithm: DigestAlgorithm, input: &[u8]) -> String {
    match algorithm {
        DigestAlgorithm::Sha256 => hex::encode(Sha256::digest(input)),
        DigestAlgorithm::Sha384 => hex::encode(Sha384::digest(input)),
        DigestAlgorithm::Sha512 => hex::encode(Sha512::digest(input)),
        DigestAlgorithm::Blake3 => hex::encode(blake3::hash(input).as_bytes()),
        DigestAlgorithm::Md5 => hex::encode(md5::Md5::digest(input)),
    }
}

pub fn hmac(algorithm: HmacAlgorithm, key: &[u8], message: &[u8]) -> Result<String, Error> {
    if key.is_empty() {
        return Err(Error::EmptyHmacKey);
    }

    let output = match algorithm {
        HmacAlgorithm::Sha256 => {
            let mut value = Hmac::<Sha256>::new_from_slice(key)
                .expect("HMAC-SHA-256 accepts keys of any non-zero length");
            value.update(message);
            hex::encode(value.finalize().into_bytes())
        }
        HmacAlgorithm::Sha512 => {
            let mut value = Hmac::<Sha512>::new_from_slice(key)
                .expect("HMAC-SHA-512 accepts keys of any non-zero length");
            value.update(message);
            hex::encode(value.finalize().into_bytes())
        }
    };
    Ok(output)
}
