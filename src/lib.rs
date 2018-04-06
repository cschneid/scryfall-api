extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

/// Base URL
const SCRYFALL_API: &str = "https://api.scryfall.com";

/// Number of milliseconds between each call, at a minimum
const SCRYFALL_DEFAULT_WAIT: i32 = 50;

struct ScryfallApi {
    base_url: String,
    last_request: Option<chrono::DateTime<chrono::Utc>>,
}

impl ScryfallApi {
    pub fn new() -> ScryfallApi {
        ScryfallApi {
            base_url: SCRYFALL_API.to_string(),
            last_request: None,
        }
    }

    pub fn run<T>(&self, req: T) -> Result<T::Response, reqwest::Error>
    where
        T: ScryfallRequest + std::fmt::Debug,
    {
        let url = format!("{}{}", self.base_url, req.path());
        let client = reqwest::ClientBuilder::new().build().unwrap();
        println!("Running {:?} - Fetching from: {:?}", req, url);
        let result = client.get(&url).send();
        result
            .and_then(|mut ok| ok.text())
            .map(|raw| T::Response::parse(&raw))
    }
}

trait ScryfallRequest {
    type Response: ScryfallResponse;
    fn path(&self) -> String;
}

trait ScryfallResponse {
    fn parse(raw: &str) -> Self;
}

type ManaCost = String;
type SetCode = String;
type SetNumber = i64; // Collectors number in set. TODO: Way too big, what is the real range?
type MultiverseId = i64; // TODO: Way too big, what is the real range?
type MtgoId = i64; // TODO: Way too big, what is the real range?
type ScryfallId = String; // TODO: Way too big, what is the real range?

/////////////////
//  Endpoints  //
/////////////////

#[derive(Debug)]
struct Sets;

impl ScryfallRequest for Sets {
    type Response = ListObject<types::Set>;

    fn path(&self) -> String {
        "/sets".into()
    }
}

struct Set(SetCode);

struct Cards; // The entire database
struct CardSearch(CardSearchFields);
struct CardNamed(String, Exact);
struct CardAutoComplete(String);
struct CardRandom;
struct CardMultiverse(MultiverseId);
struct CardMtgo(MtgoId);
struct CardInSet(SetCode, SetNumber);
struct CardScryfall(ScryfallId);

struct RulingsMultiverse(MultiverseId);
struct RulingsMtgo(MtgoId);
struct RulingsInSet(SetCode, SetNumber);
struct RulingsScryfall(ScryfallId);

struct CardSymbols;
struct ParseMana(ManaCost);

// Bulk queries
struct CatalogCardNames;
struct CatalogWordBank;
struct CatalogCreatureTypes;
struct CatalogPlaneswalkerTypes;
struct CatalogLandTypes;
struct CatalogEnchantmentTypes;
struct CatalogSpellTypes;
struct CatalogPowers;
struct CatalogToughnesses;
struct CatalogLoyalties;
struct CatalogWatermarks;

////////////////////////////
//  Search Configuration  //
////////////////////////////

struct CardSearchFields {
    uniqueness: SearchUniquenessMode,
    ordering: SearchOrdering,
    ordering_direction: SearchOrderingDirection,
}

enum Exact {
    Exact,
    Fuzzy,
}

enum SearchUniquenessMode {
    /// One copy of each card name (no matter how many printings)
    Cards,
    /// One copy for each art
    Arts,
    /// One copy for every printing
    Prints,
}

enum SearchOrdering {
    /// Sort cards by name, A → Z (default)
    Name,
    /// Sort cards by their set and collector number: oldest → newest
    Set,
    /// Sort cards by their rarity: Common → Mythic
    Rarity,
    /// Sort cards by their color and color identity: WUBRG → multicolor → colorless
    Color,
    /// Sort cards by their lowest known U.S. Dollar price: 0.01 → highest, null last
    USD,
    /// Sort cards by their lowest known TIX price: 0.01 → highest, null last
    TIX,
    /// Sort cards by their lowest known Euro price: 0.01 → highest, null last
    Eur,
    /// Sort cards by their converted mana cost: 0 → highest
    CMC,
    /// Sort cards by their power: null → highest
    Power,
    ///Sort cards by their toughness: null → highest
    Toughness,
    /// Sort cards by their EDHREC ranking: lowest → highest
    Edhrec,
    /// Sort cards by their front-side artist name: A → Z
    Artist,
}

enum SearchOrderingDirection {
    /// Scryfall will automatically choose the most inuitive direction to sort
    Auto,
    /// Sort ascending (the direction of the arrows in the previous table)
    Asc,
    /// Sort descending (flip the direction of the arrows in the previous table)
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListObject<T> {
    object: String, // Always set to 'list'
    has_more: bool,
    data: Vec<T>,
    next_page: Option<String>,
    total: Option<u32>,
    warnings: Option<Vec<String>>,
}

use serde::de::DeserializeOwned;

impl<T> ScryfallResponse for ListObject<T>
where
    T: DeserializeOwned,
{
    fn parse(raw: &str) -> ListObject<T> {
        serde_json::from_str(raw).unwrap()
    }
}

mod types {
    extern crate chrono;
    type URI = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Set {
        /// Always set to 'set'
        object: String,
        /// The unique three or four-letter code for this set.
        code: String,
        /// The unique code for this set on MTGO, which may differ from the regular code.
        mtgo_code: Option<String>,
        /// The English name of the set.
        name: String,
        /// A computer-readable classification for this set. See below.
        set_type: String,
        released_at: Option<String>, // Option<chrono::DateTime<chrono::Utc>>,
        block_code: Option<String>,
        /// Nullable The block or group name code for this set, if any.
        block: Option<String>,
        /// The set code for the parent set, if any. promo and token sets often have a parent set.
        parent_set_code: Option<String>,
        /// The number of cards in this set.
        card_count: i32,
        /// True if this set was only released on Magic Online.
        digital: bool,
        /// True if this set contains only foil cards.
        foil: bool,

        /// A URI to an SVG file for this set’s icon on Scryfall’s CDN. Hotlinking this image isn’t
        /// recommended, because it may change slightly over time. You should download it and use it
        /// locally for your particular user interface needs.
        icon_svg_uri: URI,
        /// A Scryfall API URI that you can request to begin paginating over the cards in this set.
        search_uri: URI,
        scryfall_uri: URI,
    }

    pub enum ImageVersion {
        Small,
        Normal,
        Large,
        Png,
        ArtCrop,
        BorderCrop,
    }

    pub enum ImageFace {
        Front,
        Back,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json;

        #[test]
        fn set_parse() {
            let json = r#" {
                    "object": "set",
                    "code": "ss1",
                    "name": "Signature Spellbook: Jace",
                    "uri": "https://api.scryfall.com/sets/ss1",
                    "scryfall_uri": "https://scryfall.com/sets/ss1",
                    "released_at": "2018-06-15",
                    "set_type": "spellbook",
                    "card_count": 8,
                    "digital": false,
                    "foil": false,
                    "icon_svg_uri": "https://assets.scryfall.com/assets/sets/default.svg",
                    "search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Ass1&unique=prints"
                }
            "#;

            let _set: Set = serde_json::from_str(json).expect("Parse Set JSON");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_query() {
        let a = ScryfallApi::new();
        match a.run(Sets) {
            Ok(_x) => println!("Worked"),
            Err(e) => panic!("Failed to fetch: {:?}", e),
        }
    }
}
