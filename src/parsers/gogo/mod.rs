use reqwest_impersonate::Url;

use self::anime::GogoAnime;

use super::{common::SearchResult, ExternalFile};

pub mod anime;

#[derive(Debug)]
pub struct GogoSearchResult {
    title: String,
    link: Url,
    cover: ExternalFile,
}

impl GogoSearchResult {
    pub fn new(title: String, link: String, cover: impl Into<ExternalFile>) -> Self {
        let link = Url::parse(&link).unwrap();
        let cover = cover.into();
        Self { title, link, cover }
    }
}

impl SearchResult for GogoSearchResult {
    fn title(&self) -> &str {
        &self.title
    }

    fn link(&self) -> &Url {
        &self.link
    }

    fn cover(&self) -> &ExternalFile {
        &self.cover
    }
}
