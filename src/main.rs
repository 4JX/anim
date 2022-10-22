#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// use parsers::gogo::Gogo;
use parsers::{anime::AnimeParser, gogo::Gogo};
use reqwest_impersonate::browser::ChromeVersion;
mod parsers;

fn main() {
    let client_imp = reqwest_impersonate::blocking::Client::builder()
        .chrome_builder(ChromeVersion::V106)
        .build()
        .unwrap();

    let gogo = Gogo::new();
    let shows = gogo.search_anime(&client_imp, "JoJo no Kimyou na Bouken (1993)");

    if !shows.is_empty() {
        dbg!(gogo.load_episodes(&client_imp, &shows[0]));
    }
    // for show in shows {
    //     dbg!(show);
    // }
}
