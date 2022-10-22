use reqwest_impersonate::{blocking::Client, Url};
use scraper::{Html, Selector};

use crate::parsers::{select, SearchResult};

pub(super) struct GogoAnime {
    host: Url,
}

impl GogoAnime {
    pub fn new(host: &str) -> Self {
        Self {
            host: Url::parse(host).unwrap(),
        }
    }

    pub fn search(&self, client: &Client, query: &str) -> Vec<SearchResult> {
        let mut search_url = self.host.clone();
        search_url.set_path("/search.html");
        search_url.set_query(Some(&format! {"keyword={query}"}));

        let document =
            Html::parse_document(&client.get(search_url).send().unwrap().text().unwrap());

        let search_selector = Selector::parse(".last_episodes > ul > li div.img > a").unwrap();
        let selected = document.select(&search_selector);

        selected
            .map(|element_ref| {
                dbg!(&element_ref.html());
                let e = element_ref.value();
                let title = e.attr("title").unwrap().to_string();
                let link = e.attr("href").unwrap().to_string();
                let mut img_select = element_ref.select(&select::IMG);
                let img_url = img_select.next().unwrap().value().attr("src").unwrap();
                SearchResult::new(title, link, img_url)
            })
            .collect()
    }
}
