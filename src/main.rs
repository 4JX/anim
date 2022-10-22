#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// use parsers::gogo::Gogo;
use parsers::{common::AnimeSource, gogo::Gogo};
use reqwest_impersonate::browser::ChromeVersion;
mod parsers;

fn main() {
    let client_imp = reqwest_impersonate::blocking::Client::builder()
        .chrome_builder(ChromeVersion::V106)
        .build()
        .unwrap();

    let shows = Gogo::new().search_anime(&client_imp, "JoJo no Kimyou na Bouken (1993)");

    // for show in shows {
    //     dbg!(show);
    // }
}
