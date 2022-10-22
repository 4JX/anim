use reqwest_impersonate::{blocking::Client, Url};

use super::ExternalFile;

pub trait SearchResult {
    fn title(&self) -> &str;
    fn link(&self) -> &Url;
    fn cover(&self) -> &ExternalFile;
}
