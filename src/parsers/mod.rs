use reqwest_impersonate::{header::HeaderMap, Url};

pub mod gogo;
mod select;

trait SearchResult {
    fn title(&self) -> &str;
    fn link(&self) -> &Url;
    fn cover(&self) -> &ExternalFile;
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
