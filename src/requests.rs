use super::types;
use super::types::*;
use ScryfallRequest;

#[derive(Debug)]
pub struct Sets;

#[derive(Debug)]
pub struct Set(pub SetCode);

#[derive(Debug)]
pub struct Cards; // The entire database

pub struct CardSearch(CardSearchFields);

#[derive(Debug)]
pub struct CardNamed(pub String, pub Exact);

#[derive(Debug)]
pub struct CardAutoComplete(pub String);

pub struct CardRandom;
pub struct CardMultiverse(MultiverseId);
pub struct CardMtgo(MtgoId);
pub struct CardInSet(SetCode, SetNumber);
pub struct CardScryfall(ScryfallId);

pub struct RulingsMultiverse(MultiverseId);
pub struct RulingsMtgo(MtgoId);
pub struct RulingsInSet(SetCode, SetNumber);
pub struct RulingsScryfall(ScryfallId);

pub struct CardSymbols;
pub struct ParseMana(ManaCost);

// Bulk queries
pub struct CatalogCardNames;
pub struct CatalogWordBank;
pub struct CatalogCreatureTypes;
pub struct CatalogPlaneswalkerTypes;
pub struct CatalogLandTypes;
pub struct CatalogEnchantmentTypes;
pub struct CatalogSpellTypes;
pub struct CatalogPowers;
pub struct CatalogToughnesses;
pub struct CatalogLoyalties;
pub struct CatalogWatermarks;

//////////////////////
//  Helper Structs  //
//////////////////////

struct CardSearchFields {
    uniqueness: SearchUniquenessMode,
    ordering: SearchOrdering,
    ordering_direction: SearchOrderingDirection,
}

#[derive(Debug)]
pub enum Exact {
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

/////////////
//  Impls  //
/////////////

impl ScryfallRequest for Sets {
    type Response = types::ListObject<super::types::Set>;

    fn path(&self) -> String {
        "/sets".into()
    }
}

impl ScryfallRequest for Set {
    type Response = types::Set;
    fn path(&self) -> String {
        format!("/sets/{}", self.0)
    }
}

impl ScryfallRequest for Cards {
    type Response = types::ListObject<types::Card>;

    fn path(&self) -> String {
        "/cards".into()
    }
}

impl ScryfallRequest for CardNamed {
    type Response = types::Card;

    fn path(&self) -> String {
        let exact = match self.1 {
            Exact::Exact => "exact".to_string(),
            Exact::Fuzzy => "fuzzy".to_string(),
        };
        format!("/cards/named?{}={}", exact, self.0)
    }
}

impl ScryfallRequest for CardAutoComplete {
    type Response = types::Catalog;

    fn path(&self) -> String {
        format!("/cards/autocomplete?q={}", self.0)
    }
}
