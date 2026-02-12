use std::fmt;

#[derive(Debug)]
pub enum FetchError {
    Http(ureq::Error),
    Parse(serde_json::Error),
    NotFound(u64),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Http(e) => write!(f, "HTTP error: {e}"),
            FetchError::Parse(e) => write!(f, "JSON parse error: {e}"),
            FetchError::NotFound(id) => write!(f, "sequence A{id:06} not found"),
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FetchError::Http(e) => Some(e),
            FetchError::Parse(e) => Some(e),
            FetchError::NotFound(_) => None,
        }
    }
}

impl From<ureq::Error> for FetchError {
    fn from(e: ureq::Error) -> Self {
        FetchError::Http(e)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(e: serde_json::Error) -> Self {
        FetchError::Parse(e)
    }
}
