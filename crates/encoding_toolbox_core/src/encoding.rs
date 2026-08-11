use std::str::FromStr;

use base64::Engine as _;

use crate::Error;

pub const MAX_DECODE_TEXT_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Base64,
    Base64Url,
    Base32,
    Base32Hex,
    Hex,
}

impl Encoding {
    pub const ALL: &[Self] = &[
        Self::Base64,
        Self::Base64Url,
        Self::Base32,
        Self::Base32Hex,
        Self::Hex,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Base64 => "base64",
            Self::Base64Url => "base64url",
            Self::Base32 => "base32",
            Self::Base32Hex => "base32hex",
            Self::Hex => "hex",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Base64 => "Base64",
            Self::Base64Url => "Base64URL",
            Self::Base32 => "Base32",
            Self::Base32Hex => "Base32Hex",
            Self::Hex => "Hex",
        }
    }
}

impl FromStr for Encoding {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "base64" => Ok(Self::Base64),
            "base64url" | "base64-url" => Ok(Self::Base64Url),
            "base32" => Ok(Self::Base32),
            "base32hex" | "base32-hex" => Ok(Self::Base32Hex),
            "hex" => Ok(Self::Hex),
            _ => Err(Error::UnsupportedAlgorithm(value.to_owned())),
        }
    }
}

pub fn encode(algorithm: Encoding, input: &[u8]) -> Result<String, Error> {
    let output = match algorithm {
        Encoding::Base64 => base64::engine::general_purpose::STANDARD.encode(input),
        Encoding::Base64Url => base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input),
        Encoding::Base32 => data_encoding::BASE32.encode(input),
        Encoding::Base32Hex => data_encoding::BASE32HEX.encode(input),
        Encoding::Hex => hex::encode(input),
    };
    Ok(output)
}

pub fn decode(algorithm: Encoding, input: &str) -> Result<Vec<u8>, Error> {
    if input.len() > MAX_DECODE_TEXT_BYTES {
        return Err(Error::InputTooLarge {
            max_bytes: MAX_DECODE_TEXT_BYTES,
        });
    }

    match algorithm {
        Encoding::Base64 => base64::engine::general_purpose::STANDARD
            .decode(input)
            .map_err(|_| invalid_encoding(algorithm)),
        Encoding::Base64Url => base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(input)
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(input))
            .map_err(|_| invalid_encoding(algorithm)),
        Encoding::Base32 => data_encoding::BASE32
            .decode(input.as_bytes())
            .map_err(|_| invalid_encoding(algorithm)),
        Encoding::Base32Hex => data_encoding::BASE32HEX
            .decode(input.as_bytes())
            .map_err(|_| invalid_encoding(algorithm)),
        Encoding::Hex => hex::decode(input).map_err(|_| invalid_encoding(algorithm)),
    }
}

fn invalid_encoding(algorithm: Encoding) -> Error {
    Error::InvalidEncoding {
        algorithm: algorithm.label(),
    }
}
