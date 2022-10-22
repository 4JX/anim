use reqwest_impersonate::{blocking::Client, Url};
use scraper::{Html, Selector};

use crate::parsers::{anime::Episode, select};

use super::GogoSearchResult;

pub struct GogoAnime {
    host: Url,
}

impl GogoAnime {
    pub fn new(host: &str) -> Self {
        Self {
            host: Url::parse(host).unwrap(),
        }
    }

    pub fn search(&self, client: &Client, query: &str) -> Vec<GogoSearchResult> {
        let mut search_url = self.host.clone();
        search_url.set_path("/search.html");
        search_url.set_query(Some(&format! {"keyword={query}"}));

        let document =
            Html::parse_document(&client.get(search_url).send().unwrap().text().unwrap());

        let search_selector = Selector::parse(".last_episodes > ul > li div.img > a").unwrap();
        let selected = document.select(&search_selector);

        selected
            .map(|element_ref| {
                let e = element_ref.value();

                let title = e.attr("title").unwrap().to_string();

                let link_path = e.attr("href").unwrap().to_string();
                let mut link = self.host.clone();
                link.set_path(&link_path);

                let mut img_select = element_ref.select(&select::IMG);
                let img_url = img_select.next().unwrap().value().attr("src").unwrap();

                GogoSearchResult::new(title, link, img_url)
            })
            .collect()
    }

    pub fn load_episodes(&self, client: &Client, link: &Url) -> Vec<Episode> {
        let show_body = client.get(link.clone()).send().unwrap().text().unwrap();
        let body_html = Html::parse_document(&show_body);

        let last_ep_selector = Selector::parse("ul#episode_page > li:last-child > a").unwrap();
        let last_episode = body_html
            .select(&last_ep_selector)
            .take(1)
            .next()
            .unwrap()
            .value()
            .attr("ep_end")
            .unwrap();

        let anime_id_selector = Selector::parse("input#movie_id").unwrap();
        let anime_id = body_html
            .select(&anime_id_selector)
            .take(1)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap();

        let ep_list = client.get(&format! {"https://ajax.gogo-load.com/ajax/load-list-episode?ep_start=0&ep_end={last_episode}&id={anime_id}"}).send().unwrap().text().unwrap();
        let list_selector = Selector::parse("ul > li > a").unwrap();
        let ep_list_html = Html::parse_document(&ep_list);

        let mut list = ep_list_html
            .select(&list_selector)
            .map(|ep| {
                let num_selector = Selector::parse(".name").unwrap();
                let num = ep
                    .select(&num_selector)
                    .next()
                    .unwrap()
                    .text()
                    .enumerate()
                    .filter_map(|(i, e)| if i == 1 { Some(e) } else { None })
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .replace(" ", "");

                let ep_path = ep.value().attr("href").unwrap();
                let mut ep_link = self.host.clone();
                ep_link.set_path(ep_path);

                Episode::new(num, ep_link)
            })
            .collect::<Vec<Episode>>();

        list.reverse();

        list
    }
}
