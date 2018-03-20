extern crate chrono;

/// Base URL
const SCRYFALL_API: &str = "https://api.scryfall.com";

/// Number of milliseconds between each call, at a minimum
const SCRYFALL_DEFAULT_WAIT: i32 = 50;

type URI = String;

struct ScryfallApi {
    last_request: Option<chrono::DateTime<chrono::Utc>>,
}

impl ScryfallApi {
    pub fn new() -> ScryfallApi {
        ScryfallApi { last_request: None }
    }
}

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

type ManaCost = String;
type SetCode = String;
type SetNumber = i64; // Collectors number in set. TODO: Way too big, what is the real range?
type MultiverseId = i64; // TODO: Way too big, what is the real range?
type MtgoId = i64; // TODO: Way too big, what is the real range?
type ScryfallId = String; // TODO: Way too big, what is the real range?

enum ScryfallEndpoint {
    Sets,
    Set(SetCode),

    Cards, // The entire database
    CardSearch(CardSearchFields),
    CardNamed(String, Exact),
    CardAutoComplete(String),
    CardRandom,
    CardMultiverse(MultiverseId),
    CardMtgo(MtgoId),
    CardInSet(SetCode, SetNumber),
    CardScryfall(ScryfallId),

    RulingsMultiverse(MultiverseId),
    RulingsMtgo(MtgoId),
    RulingsInSet(SetCode, SetNumber),
    RulingsScryfall(ScryfallId),

    CardSymbols,
    ParseMana(ManaCost),

    // Bulk queries
    CatalogCardNames,
    CatalogWordBank,
    CatalogCreatureTypes,
    CatalogPlaneswalkerTypes,
    CatalogLandTypes,
    CatalogEnchantmentTypes,
    CatalogSpellTypes,
    CatalogPowers,
    CatalogToughnesses,
    CatalogLoyalties,
    CatalogWatermarks,
}

struct Set {
    /// The unique three or four-letter code for this set.
    code: String,
    /// The unique code for this set on MTGO, which may differ from the regular code.
    mtgo_code: String,
    /// The English name of the set.
    name: String,
    /// A computer-readable classification for this set. See below.
    set_type: String,
    released_at: Option<chrono::Date<chrono::Utc>>,
    block_code: Option<String>,
    /// Nullable The block or group name code for this set, if any.
    block: Option<String>,
    /// The set code for the parent set, if any. promo and token sets often have a parent set.
    parent_set_code: String,
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
}

enum ImageVersion {
    Small,
    Normal,
    Large,
    Png,
    ArtCrop,
    BorderCrop,
}

enum ImageFace {
    Front,
    Back,
}
