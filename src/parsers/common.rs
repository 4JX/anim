use reqwest_impersonate::Url;

use super::ExternalFile;

pub trait SearchResult {
    fn title(&self) -> &str;
    fn link(&self) -> &Url;
    fn cover(&self) -> &ExternalFile;
}
