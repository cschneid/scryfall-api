use super::ScryfallResponse;
use serde_json;

extern crate chrono;
type URI = String;

pub type ManaCost = String;
pub type SetCode = String;
pub type SetNumber = i64; // Collectors number in set. TODO: Way too big, what is the real range?
pub type MultiverseId = i64; // TODO: Way too big, what is the real range?
pub type MtgoId = i64; // TODO: Way too big, what is the real range?
pub type ScryfallId = String; // TODO: Way too big, what is the real range?

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    /// Always set to 'set'
    object: String,
    /// The unique three or four-letter code for this set.
    pub code: String,
    /// The unique code for this set on MTGO, which may differ from the regular code.
    pub mtgo_code: Option<String>,
    /// The English name of the set.
    pub name: String,
    /// A computer-readable classification for this set. See below.
    pub set_type: String,
    pub released_at: Option<String>, // Option<chrono::DateTime<chrono::Utc>>,
    pub block_code: Option<String>,
    /// Nullable The block or group name code for this set, if any.
    pub block: Option<String>,
    /// The set code for the parent set, if any. promo and token sets often have a parent set.
    pub parent_set_code: Option<String>,
    /// The number of cards in this set.
    pub card_count: i32,
    /// True if this set was only released on Magic Online.
    pub digital: bool,
    /// True if this set contains only foil cards.
    pub foil: bool,

    /// A URI to an SVG file for this set’s icon on Scryfall’s CDN. Hotlinking this image isn’t
    /// recommended, because it may change slightly over time. You should download it and use it
    /// locally for your particular user interface needs.
    pub icon_svg_uri: URI,
    /// A Scryfall API URI that you can request to begin paginating over the cards in this set.
    pub search_uri: URI,
    pub scryfall_uri: URI,
}

use std::collections::HashMap;
type Color = String;
type Colors = Vec<Color>;
type Legalities = HashMap<String, String>;
type Images = HashMap<String, URI>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: String,        // 	UUID		A unique ID for this card in Scryfall’s database.
    pub oracle_id: String, // 	UUID		A unique ID for this card’s oracle identity. This value is consistent across reprinted card editions, and unique among different cards with the same name (tokens, Unstable variants, etc).
    pub multiverse_ids: Option<Vec<u64>>, // 	Array	Nullable This card’s multiverse IDs on Gatherer, if any, as an array of integers. Note that Scryfall includes many promo cards, tokens, and other esoteric objects that do not have these identifiers.
    pub mtgo_id: Option<MtgoId>, // 	Integer	Nullable This card’s Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not available on Magic Online and do not have this ID.
    pub mtgo_foil_id: Option<MtgoId>, // 	Integer	Nullable This card’s foil Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not available on Magic Online and do not have this ID.
    pub uri: URI,                     //	URI		A link to this card object on Scryfall’s API.
    pub scryfall_uri: URI, //	URI		A link to this card’s permapage on Scryfall’s website.
    pub prints_search_uri: URI, //	URI		A link to where you can begin paginating all re/prints for this card on Scryfall’s API.
    pub rulings_uri: URI,       // 	URI		A link to this card’s rulings on Scryfall’s API.
    pub name: String, //String		The name of this card. If this card has multiple faces, this field will contain both names separated by ␣//␣.
    pub layout: String, //String		A computer-readable designation for this card’s layout. See the layout article.
    pub cmc: f64, //Decimal		The card’s converted mana cost. Note that some funny cards have fractional mana costs.
    pub type_line: String, //String		The type line of this card.
    pub oracle_text: Option<String>, //String	Nullable The Oracle text for this card, if any.
    pub mana_cost: String, //String		The mana cost for this card. This value will be any empty string "" if the cost is absent. Remember that per the game rules, a missing mana cost and a mana cost of {0} are different values.
    pub power: Option<String>, //String	Nullable This card’s power, if any. Note that some cards have powers that are not numeric, such as *.
    pub toughness: Option<String>, //String	Nullable This card’s toughness, if any. Note that some cards have toughnesses that are not numeric, such as *.
    pub loyalty: Option<String>, //String	Nullable This loyalty if any. Note that some cards have loyalties that are not numeric, such as X.
    pub life_modifier: Option<String>, //String	Nullable This card’s life modifier, if it is Vanguard card. This value will contain a delta, such as +2.
    pub hand_modifier: Option<String>, //String	Nullable This card’s hand modifier, if it is Vanguard card. This value will contain a delta, such as -1.
    pub colors: Colors,                //Colors		This card’s colors.
    pub color_indicator: Option<Colors>, //Colors	Nullable The colors in this card’s color indicator, if any. A null value for this field indicates the card does not have one.
    pub color_identity: Colors,          //Colors		This card’s color identity.
    pub all_parts: Option<Vec<String>>, //Array	Nullable If this card is closely related to other cards, this property will be an array with.
    pub card_faces: Option<Vec<String>>, //Array	Nullable An array of Card Face objects, if this card is multifaced.
    pub legalities: Legalities,          //Object		An object describing the legality of this card.
    pub reserved: bool,                  //Boolean		True if this card is on the Reserved List.
    pub edhrec_rank: Option<i64>, //Integer	Nullable This card’s overall rank/popularity on EDHREC. Not all carsd are ranked.
    pub set: String,              //String		This card’s set code.
    pub set_name: String,         //String		This card’s full set name.
    pub collector_number: String, //String		This card’s collector number. Note that collector numbers can contain non-numeric characters, such as letters or ★.
    pub set_search_uri: URI, //URI		A link to where you can begin paginating this card’s set on the Scryfall API.
    pub scryfall_set_uri: URI, //URI		A link to this card’s set on Scryfall’s website.
    pub image_uris: Option<Images>, //Object	Nullable An object listing available imagery for this card. See the [Card Imagery](#) article for more information.
    pub highres_image: bool,        //Boolean		True if this card’s imagery is high resolution.
    pub reprint: bool,              //Boolean		True if this card is a reprint.
    pub digital: bool,              //Boolean		True if this is a digital card on Magic Online.
    pub rarity: String, //String		This card’s rarity. One of common, uncommon, rare, or mythic.
    pub flavor_text: Option<String>, //String	Nullable The flavor text, if any.
    pub artist: Option<String>, //String	Nullable The name of the illustrator of this card. Newly spoiled cards may not have this field yet.
    pub illustration_id: Option<String>, //UUID	Nullable A unique identifier for the card artwork that remains consistent across reprints. Newly spoiled cards may not have this field yet.
    pub frame: String,                   //String		This card’s frame layout. See.
    pub full_art: bool, //Boolean		True if this card’s artwork is larger than normal.
    pub watermark: Option<String>, //String	Nullable This card’s watermark, if any.
    pub border_color: String, //String		This card’s border color: black, borderless, gold, silver, or white.
    pub story_spotlight_number: Option<u64>, //Integer	Nullable This card’s story spotlight number, if any.
    pub story_spotlight_uri: Option<URI>, //URI	Nullable A URL to this cards’s story article, if any.
    pub timeshifted: bool,                //Boolean		True if this card is timeshifted.
    pub colorshifted: bool,               //Boolean		Ture if this card is colorshifted.
    pub futureshifted: bool,              //Boolean		True if this card is from the future.
                                          // Card Face
                                          // name: String, //String		The name of this particular face.
                                          // type_line: String, //String		The type line of this particular face.
                                          // oracle_text: Option<String>, //String	Nullable The Oracle text for this face, if any.
                                          // mana_cost: String, //String		The mana cost for this face. This value will be any empty string "" if the cost is absent. Remember that per the game rules, a missing mana cost and a mana cost of {0} are different values.
                                          // colors: Colors, //Colors		This face’s colors.
                                          // color_indicator //Colors	Nullable The colors in this face’s color indicator, if any.
                                          // power //String	Nullable This face’s power, if any. Note that some cards have powers that are not numeric, such as *.
                                          // toughness //String	Nullable This face’s toughness, if any.
                                          // loyalty //String	Nullable This face’s loyalty, if any.
                                          // flavor_text //String	Nullable The flavor text printed on this face, if any.
                                          // illustration_id //UUID	Nullable A unique identifier for the card face artwork that remains consistent across reprints. Newly spoiled cards may not have this field yet.
                                          // image_uris //Object	Nullable An object providing URIs to imagery for this face, if this is a double-sided card. If this card is not double-sided, then the image_uris property will be part of the parent object instead.
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObject<T> {
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

impl ScryfallResponse for Set {
    fn parse(raw: &str) -> Set {
        serde_json::from_str(raw).unwrap()
    }
}

impl ScryfallResponse for Card {
    fn parse(raw: &str) -> Card {
        serde_json::from_str(raw).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn list_object_parse() {
        let json = include_str!("../testcases/list_object_sets.json");
        let _list: ListObject<Set> = serde_json::from_str(json).expect("Parse ListObject<Set>");
    }

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

    #[test]
    fn card_parse() {
        let json = r#"
                {
                    "object": "card",
                    "id": "8cae1a42-052e-4110-9afc-d3ec83b7c8a9",
                    "oracle_id": "67362406-b1ca-49e2-800d-9050bfe8742a",
                    "multiverse_ids": [
                        442054
                    ],
                    "mtgo_id": 67044,
                    "mtgo_foil_id": 67045,
                    "name": "Merfolk Looter",
                    "uri": "https://api.scryfall.com/cards/a25/65",
                    "scryfall_uri": "https://scryfall.com/card/a25/65?utm_source=api",
                    "layout": "normal",
                    "highres_image": true,
                    "image_uris": {
                        "small": "https://img.scryfall.com/cards/small/en/a25/65.jpg?1521725642",
                        "normal": "https://img.scryfall.com/cards/normal/en/a25/65.jpg?1521725642",
                        "large": "https://img.scryfall.com/cards/large/en/a25/65.jpg?1521725642",
                        "png": "https://img.scryfall.com/cards/png/en/a25/65.png?1521725642",
                        "art_crop": "https://img.scryfall.com/cards/art_crop/en/a25/65.jpg?1521725642",
                        "border_crop": "https://img.scryfall.com/cards/border_crop/en/a25/65.jpg?1521725642"
                    },
                    "cmc": 2,
                    "type_line": "Creature — Merfolk Rogue",
                    "oracle_text": "{T}: Draw a card, then discard a card.",
                    "mana_cost": "{1}{U}",
                    "power": "1",
                    "toughness": "1",
                    "colors": [
                        "U"
                    ],
                    "color_identity": [
                        "U"
                    ],
                    "legalities": {
                        "standard": "not_legal",
                        "future": "not_legal",
                        "frontier": "not_legal",
                        "modern": "legal",
                        "legacy": "legal",
                        "pauper": "legal",
                        "vintage": "legal",
                        "penny": "legal",
                        "commander": "legal",
                        "1v1": "legal",
                        "duel": "legal",
                        "brawl": "not_legal"
                    },
                    "reserved": false,
                    "reprint": true,
                    "set": "a25",
                    "set_name": "Masters 25",
                    "set_uri": "https://api.scryfall.com/sets/a25",
                    "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Aa25&unique=prints",
                    "scryfall_set_uri": "https://scryfall.com/sets/a25?utm_source=api",
                    "rulings_uri": "https://api.scryfall.com/cards/a25/65/rulings",
                    "prints_search_uri": "https://api.scryfall.com/cards/search?order=set&q=%21%E2%80%9CMerfolk+Looter%E2%80%9D&unique=prints",
                    "collector_number": "65",
                    "digital": false,
                    "rarity": "uncommon",
                    "watermark": "set",
                    "flavor_text": "Merfolk don't always know what they're looking for, but they're certain once they find it.",
                    "illustration_id": "c192fa94-2420-4c2d-a6fb-d103fb42a925",
                    "artist": "Tristan Elwell",
                    "frame": "2015",
                    "full_art": false,
                    "border_color": "black",
                    "timeshifted": false,
                    "colorshifted": false,
                    "futureshifted": false,
                    "edhrec_rank": 3927,
                    "usd": "0.07",
                    "tix": "0.02",
                    "eur": "0.05",
                    "related_uris": {
                        "gatherer": "http://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=442054",
                        "tcgplayer_decks": "http://decks.tcgplayer.com/magic/deck/search?contains=Merfolk+Looter&page=1&partner=Scryfall",
                        "edhrec": "http://edhrec.com/route/?cc=Merfolk+Looter",
                        "mtgtop8": "http://mtgtop8.com/search?MD_check=1&SB_check=1&cards=Merfolk+Looter"
                    },
                    "purchase_uris": {
                        "amazon": "https://www.amazon.com/gp/search?ie=UTF8&index=toys-and-games&keywords=Merfolk+Looter&tag=scryfall-20",
                        "ebay": "http://rover.ebay.com/rover/1/711-53200-19255-0/1?campid=5337966903&icep_catId=19107&icep_ff3=10&icep_sortBy=12&icep_uq=Merfolk+Looter&icep_vectorid=229466&ipn=psmain&kw=lg&kwid=902099&mtid=824&pub=5575230669&toolid=10001",
                        "tcgplayer": "https://scryfall.com/s/tcgplayer/161494",
                        "magiccardmarket": "https://scryfall.com/s/mcm/319234",
                        "cardhoarder": "https://www.cardhoarder.com/cards/67044?affiliate_id=scryfall&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall",
                        "card_kingdom": "https://www.cardkingdom.com/catalog/item/217162?partner=scryfall&utm_campaign=affiliate&utm_medium=scryfall&utm_source=scryfall",
                        "mtgo_traders": "http://www.mtgotraders.com/deck/ref.php?id=67044&referral=scryfall",
                        "coolstuffinc": "https://scryfall.com/s/coolstuffinc/4340666"
                    }
                }
            "#;
        let _card: Card = serde_json::from_str(json).expect("Parse Card JSON");
    }
}
