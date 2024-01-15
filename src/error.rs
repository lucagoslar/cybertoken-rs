/// Possible token parsing errors
#[derive(Debug, Clone, Copy)]
pub enum CybertokenError {
    IncludesUnderscore,
    TokenLength,
    VersionMismatch,
    Base62DecodingError,
}

impl std::error::Error for CybertokenError {}

impl std::fmt::Display for CybertokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CybertokenError::IncludesUnderscore => write!(f, "Missing underscore"),
            CybertokenError::TokenLength => write!(f, "Invalid token length"),
            CybertokenError::VersionMismatch => write!(f, "Token version mismatch"),
            CybertokenError::Base62DecodingError => write!(f, "Found invalid Base62 data"),
        }
    }
}
