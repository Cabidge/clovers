use std::{convert::Infallible, str::FromStr};

use base64ct::Encoding;
use blake2::{Blake2s256, Digest};

pub struct Poster {
    pub name: String,
    hash: Option<Box<[u8]>>,
}

impl Poster {
    pub const DEFAULT_NAME: &str = "Anonymous";

    pub fn new() -> Self {
        Self::with_name(Self::DEFAULT_NAME)
    }

    pub fn with_secret(secret: &str) -> Self {
        Self::with_name_and_secret(Self::DEFAULT_NAME, secret)
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hash: None,
        }
    }

    pub fn with_name_and_secret(name: impl Into<String>, secret: &str) -> Self {
        let name = name.into();

        let hash = Blake2s256::new()
            .chain_update(secret)
            .chain_update("#clovers#")
            .chain_update(&name)
            .finalize()
            .to_vec()
            .into_boxed_slice();

        Self {
            name,
            hash: Some(hash),
        }
    }

    pub fn hash(&self) -> Option<String> {
        self.hash
            .as_ref()
            .map(|hash| base64ct::Base64UrlUnpadded::encode_string(hash))
    }
}

impl FromStr for Poster {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, secret) = s
            .split_once('#')
            .map(|(name, secret)| (name, Some(secret)))
            .unwrap_or((s, None));

        // If the name is empty, we'll use the default name.
        let poster = match (name, secret) {
            ("", None) => Self::new(),
            ("", Some(secret)) => Self::with_secret(secret),
            (name, None) => Self::with_name(name),
            (name, Some(secret)) => Self::with_name_and_secret(name, secret),
        };

        Ok(poster)
    }
}
