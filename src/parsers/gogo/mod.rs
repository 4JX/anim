use self::anime::GogoAnime;

use super::SearchResult;

mod anime;

pub struct Gogo {
    anime: GogoAnime,
}

pub struct GogoShowResponse(SearchResult);
