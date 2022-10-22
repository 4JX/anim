use reqwest_impersonate::{blocking::Client, Url};

use super::{common::SearchResult, ExternalFile};

/// Represents a source that is able to obtain videos based on a given query
pub trait AnimeParser {
    type AnimeSearchResult: SearchResult;

    /// Search for a specific query on the site
    fn search_anime(&self, client: &Client, query: &str) -> Vec<Self::AnimeSearchResult>;

    /// Load the episodes from a certain result
    fn load_episodes(&self, client: &Client, result: &Self::AnimeSearchResult) -> Vec<Episode>;
}

#[derive(Debug)]
pub struct Episode {
    /// The episode number
    number: String,

    /// A link to the page that holds the video list for further extraction
    url: Url,

    title: Option<String>,
    thumbnail: Option<ExternalFile>,
    description: Option<String>,
    is_filler: bool,
}

impl Episode {
    pub fn new(number: String, url: Url) -> Self {
        Self {
            number,
            url,
            title: None,
            thumbnail: None,
            description: None,
            is_filler: false,
        }
    }
}
