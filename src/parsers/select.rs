use once_cell::sync::Lazy;
use scraper::Selector;

pub static IMG: Lazy<Selector> = Lazy::new(|| Selector::parse("img").unwrap());
