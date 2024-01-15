use crate::error::CybertokenError;
use crc32fast::hash;
use rand::{thread_rng, RngCore};

pub const BASE62_ALPHABET: &'static str =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// The contents of a cybertoken
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CybertokenContents<'a> {
    pub prefix_without_underscore: &'a str,
    pub secret: Vec<u8>,
    pub version: usize,
    pub supplied_checksum: Vec<u8>,
    pub actual_checksum: Vec<u8>,
    pub is_syntactically_valid: bool,
}

/// A helper struct to persist a cybertoken configuration
#[derive(Debug, Clone, Copy)]
pub struct Cybertoken<'a> {
    pub prefix: &'a str,
    pub version: u8,
    pub entropy_bytes: usize,
}

impl<'a> Cybertoken<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self {
            prefix,
            version: 0,
            entropy_bytes: 23,
        }
    }

    pub fn generate_token(&self) -> String {
        let mut token = self.prefix.to_owned();

        let mut buf = vec![0_u8; self.entropy_bytes + 4];
        let mut entropy_bytes = vec![0_u8; self.entropy_bytes];
        thread_rng().fill_bytes(&mut entropy_bytes);
        entropy_bytes[self.entropy_bytes - 1] = 0;

        for (index, entropy_bytes) in entropy_bytes.iter().enumerate() {
            buf[index] = *entropy_bytes;
        }

        let checksum = hash(&entropy_bytes);

        let checksum = checksum.to_be_bytes();
        for (index, checksum) in checksum.iter().enumerate() {
            buf[index + entropy_bytes.len()] = *checksum;
        }

        token.push_str("_");
        token.push_str(&base_x::encode(BASE62_ALPHABET, &buf));

        token
    }

    pub fn parse_token_data(
        &self,
        token: &'a str,
    ) -> Result<CybertokenContents<'a>, CybertokenError> {
        let delimeter = match token.find("_") {
            Some(i) => i,
            None => return Err(CybertokenError::IncludesUnderscore),
        };

        let prefix = &token[..delimeter];
        let token = &token[delimeter + 1..];

        let raw_token = base_x::decode(BASE62_ALPHABET, token)
            .map_err(|_| CybertokenError::Base62DecodingError)?;

        if raw_token.len() < 6 {
            return Err(CybertokenError::TokenLength);
        }

        let checksum = &raw_token[raw_token.len() - 4..];
        let version = &raw_token[raw_token.len() - 5];
        let entropy_bytes = &raw_token[..raw_token.len() - 5];

        if version != &self.version {
            return Err(CybertokenError::VersionMismatch);
        }

        let actual_checksum = &hash(&raw_token[..raw_token.len() - 4]).to_be_bytes();

        Ok(CybertokenContents {
            prefix_without_underscore: prefix,
            secret: entropy_bytes.to_vec(),
            version: *version as usize,
            supplied_checksum: checksum.to_vec(),
            actual_checksum: actual_checksum.to_vec(),
            is_syntactically_valid: (entropy_bytes.len() > 0) && (actual_checksum == checksum),
        })
    }

    pub fn is_token_string(&self, token: &str) -> bool {
        match self.parse_token_data(token) {
            Ok(token_data) => token_data.is_syntactically_valid,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_token() {
        let cybertoken = Cybertoken::new("zugriff");
        let _ = cybertoken.generate_token();
    }

    #[test]
    fn is_token_string() {
        let cybertoken = Cybertoken::new("zugriff");
        assert_eq!(
            cybertoken.is_token_string("zugriff_icnocrRLDoZ3uCPosLA0277hQ58ob379X43U"),
            true
        );
    }

    #[test]
    fn parse_token_data() {
        let cybertoken = Cybertoken::new("zugriff");

        assert_eq!(
            cybertoken
                .parse_token_data("zugriff_icnocrRLDoZ3uCPosLA0277hQ58ob379X43U")
                .unwrap(),
            CybertokenContents {
                prefix_without_underscore: "zugriff",
                secret: vec![
                    58, 193, 176, 144, 157, 249, 16, 243, 200, 153, 174, 106, 124, 88, 188, 66,
                    150, 21, 251, 25, 217, 216
                ],
                version: 0,
                supplied_checksum: vec![65, 232, 134, 208],
                actual_checksum: vec![65, 232, 134, 208],
                is_syntactically_valid: true
            }
        );
    }

    #[test]
    fn f_is_token_string() {
        let cybertoken = Cybertoken::new("zugriff");
        assert_eq!(
            cybertoken.is_token_string("zugriff_icnocrRLDoZ3uCPosLA0277hQ58ob379X43B"),
            false
        );
    }
}
