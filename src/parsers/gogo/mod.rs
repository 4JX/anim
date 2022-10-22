use reqwest_impersonate::{blocking::Client, Url};

use self::anime::GogoAnime;

use super::{
    anime::{AnimeParser, Episode},
    common::SearchResult,
    ExternalFile,
};

mod anime;

pub struct Gogo {
    anime: GogoAnime,
}

impl Gogo {
    pub fn new() -> Self {
        Self {
            anime: GogoAnime::new("https://gogoanime.sk"),
        }
    }
}

impl AnimeParser for Gogo {
    type AnimeSearchResult = GogoSearchResult;

    fn search_anime(&self, client: &Client, query: &str) -> Vec<Self::AnimeSearchResult> {
        self.anime.search(client, query)
    }

    fn load_episodes(&self, client: &Client, result: &Self::AnimeSearchResult) -> Vec<Episode> {
        let link = result.link();
        self.anime.load_episodes(client, link)
    }
}

#[derive(Debug)]
pub struct GogoSearchResult {
    title: String,
    link: Url,
    cover: ExternalFile,
}

impl GogoSearchResult {
    pub fn new(title: String, link: Url, cover: impl Into<ExternalFile>) -> Self {
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
