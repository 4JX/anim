#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// use parsers::gogo::Gogo;
use parsers::gogo::anime::GogoAnime;
use reqwest_impersonate::browser::ChromeVersion;
mod parsers;

fn main() {
    let client_imp = reqwest_impersonate::blocking::Client::builder()
        .chrome_builder(ChromeVersion::V106)
        .build()
        .unwrap();

    let shows = GogoAnime::new("https://gogoanime.sk")
        .search(&client_imp, "JoJo no Kimyou na Bouken (1993)");

    // for show in shows {
    //     dbg!(show);
    // }
}
