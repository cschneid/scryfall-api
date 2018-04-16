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
    pub foil_only: bool,

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
    /// 	UUID		A unique ID for this card in Scryfall’s database.
    pub id: String,
    /// 	UUID		A unique ID for this card’s oracle identity. This value is consistent across reprinted card editions, and unique among different cards with the same name (tokens, Unstable variants, etc).
    pub oracle_id: String,
    /// 	Array	Nullable This card’s multiverse IDs on Gatherer, if any, as an array of integers. Note that Scryfall includes many promo cards, tokens, and other esoteric objects that do not have these identifiers.
    pub multiverse_ids: Option<Vec<u64>>,
    /// 	Integer	Nullable This card’s Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not available on Magic Online and do not have this ID.
    pub mtgo_id: Option<MtgoId>,
    /// 	Integer	Nullable This card’s foil Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not available on Magic Online and do not have this ID.
    pub mtgo_foil_id: Option<MtgoId>,
    ///	URI		A link to this card object on Scryfall’s API.
    pub uri: URI,
    ///	URI		A link to this card’s permapage on Scryfall’s website.
    pub scryfall_uri: URI,
    ///	URI		A link to where you can begin paginating all re/prints for this card on Scryfall’s API.
    pub prints_search_uri: URI,
    /// 	URI		A link to this card’s rulings on Scryfall’s API.
    pub rulings_uri: URI,
    ///String		The name of this card. If this card has multiple faces, this field will contain both names separated by ␣//␣.
    pub name: String,
    ///String		A computer-readable designation for this card’s layout. See the layout article.
    pub layout: String,
    ///Decimal		The card’s converted mana cost. Note that some funny cards have fractional mana costs.
    pub cmc: f64,
    ///String		The type line of this card.
    pub type_line: String,
    ///String	Nullable The Oracle text for this card, if any.
    pub oracle_text: Option<String>,
    ///String		The mana cost for this card. This value will be any empty string "" if the cost is absent. Remember that per the game rules, a missing mana cost and a mana cost of {0} are different values.
    pub mana_cost: String,
    ///String	Nullable This card’s power, if any. Note that some cards have powers that are not numeric, such as *.
    pub power: Option<String>,
    ///String	Nullable This card’s toughness, if any. Note that some cards have toughnesses that are not numeric, such as *.
    pub toughness: Option<String>,
    ///String	Nullable This loyalty if any. Note that some cards have loyalties that are not numeric, such as X.
    pub loyalty: Option<String>,
    ///String	Nullable This card’s life modifier, if it is Vanguard card. This value will contain a delta, such as +2.
    pub life_modifier: Option<String>,
    ///String	Nullable This card’s hand modifier, if it is Vanguard card. This value will contain a delta, such as -1.
    pub hand_modifier: Option<String>,
    ///Colors		This card’s colors.
    pub colors: Colors,
    ///Colors	Nullable The colors in this card’s color indicator, if any. A null value for this field indicates the card does not have one.
    pub color_indicator: Option<Colors>,
    ///Colors		This card’s color identity.
    pub color_identity: Colors,
    ///Array	Nullable If this card is closely related to other cards, this property will be an array with.
    pub all_parts: Option<Vec<String>>,
    ///Array	Nullable An array of Card Face objects, if this card is multifaced.
    pub card_faces: Option<Vec<String>>,
    ///Object		An object describing the legality of this card.
    pub legalities: Legalities,
    ///Boolean		True if this card is on the Reserved List.
    pub reserved: bool,
    ///Integer	Nullable This card’s overall rank/popularity on EDHREC. Not all carsd are ranked.
    pub edhrec_rank: Option<i64>,
    ///String		This card’s set code.
    pub set: String,
    ///String		This card’s full set name.
    pub set_name: String,
    ///String		This card’s collector number. Note that collector numbers can contain non-numeric characters, such as letters or ★.
    pub collector_number: String,
    ///URI		A link to where you can begin paginating this card’s set on the Scryfall API.
    pub set_search_uri: URI,
    ///URI		A link to this card’s set on Scryfall’s website.
    pub scryfall_set_uri: URI,
    ///Object	Nullable An object listing available imagery for this card. See the [Card Imagery](#) article for more information.
    pub image_uris: Option<Images>,
    ///Boolean		True if this card’s imagery is high resolution.
    pub highres_image: bool,
    ///Boolean		True if this card is a reprint.
    pub reprint: bool,
    ///Boolean		True if this is a digital card on Magic Online.
    pub digital: bool,
    ///String		This card’s rarity. One of common, uncommon, rare, or mythic.
    pub rarity: String,
    ///String	Nullable The flavor text, if any.
    pub flavor_text: Option<String>,
    ///String	Nullable The name of the illustrator of this card. Newly spoiled cards may not have this field yet.
    pub artist: Option<String>,
    ///UUID	Nullable A unique identifier for the card artwork that remains consistent across reprints. Newly spoiled cards may not have this field yet.
    pub illustration_id: Option<String>,
    ///String		This card’s frame layout. See.
    pub frame: String,
    ///Boolean		True if this card’s artwork is larger than normal.
    pub full_art: bool,
    ///String	Nullable This card’s watermark, if any.
    pub watermark: Option<String>,
    ///String		This card’s border color: black, borderless, gold, silver, or white.
    pub border_color: String,
    ///Integer	Nullable This card’s story spotlight number, if any.
    pub story_spotlight_number: Option<u64>,
    ///URI	Nullable A URL to this cards’s story article, if any.
    pub story_spotlight_uri: Option<URI>,
    ///Boolean		True if this card is timeshifted.
    pub timeshifted: bool,
    ///Boolean		Ture if this card is colorshifted.
    pub colorshifted: bool,
    ///Boolean		True if this card is from the future.
    pub futureshifted: bool,
    // CardFaces

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
        let json = include_str!("../testcases/set.json");
        let _set: Set = serde_json::from_str(json).expect("Parse Set JSON");
    }

    #[test]
    fn card_parse() {
        let json = include_str!("../testcases/card.json");
        let _card: Card = serde_json::from_str(json).expect("Parse Card JSON");
    }
}
