use reqwest_impersonate::{header::HeaderMap, Url};

pub mod gogo;
mod select;

/// A search result containing information about a show
#[derive(Debug)]
pub struct SearchResult {
    name: String,
    link: String,
    cover: ExternalFile,
}

impl SearchResult {
    pub fn new(name: String, link: String, cover: impl Into<ExternalFile>) -> Self {
        let cover = cover.into();
        Self { name, link, cover }
    }
}

/// Represents a file hosted on a website
#[derive(Debug)]
pub struct ExternalFile {
    url: Url,
    headers: Option<HeaderMap>,
}

impl ExternalFile {
    pub fn new(url: &str) -> Self {
        Self {
            url: Url::parse(url).unwrap(),
            headers: None,
        }
    }
}

impl From<&str> for ExternalFile {
    fn from(str: &str) -> Self {
        ExternalFile::new(str)
    }
}
