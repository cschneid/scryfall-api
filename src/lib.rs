extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod requests;
mod types;

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
        result.and_then(|mut ok| ok.text()).map(|raw| {
            println!("{:?}", raw);
            T::Response::parse(&raw)
        })
    }
}

trait ScryfallRequest {
    type Response: ScryfallResponse;
    fn path(&self) -> String;
}

trait ScryfallResponse {
    fn parse(raw: &str) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_request() {
        let a = ScryfallApi::new();
        match a.run(requests::Sets) {
            Ok(_x) => println!("Worked"),
            Err(e) => panic!("Failed to fetch: {:?}", e),
        }
    }

    #[test]
    fn set_request() {
        let a = ScryfallApi::new();
        match a.run(requests::Set("zen".to_string())) {
            Ok(set) => assert_eq!("Zendikar".to_string(), set.name),
            Err(e) => panic!("Failed to fetch: {:?}", e),
        }
    }

    #[test]
    fn cards_request() {
        let a = ScryfallApi::new();
        match a.run(requests::Cards) {
            Ok(_list) => println!("Worked"),
            Err(e) => panic!("Failed to fetch: {:?}", e),
        }
    }

    // TODO: How to handle negative search results?
    // {"object": "error",
    //  "code": "not_found",
    //  "type": "ambiguous",
    //  "status": 404,
    //  "details": "Too many cards match ambiguous name “aus com”. Add more words to refine your search."
    #[test]
    fn card_named_request() {
        let a = ScryfallApi::new();
        match a.run(requests::CardNamed(
            "aust com".to_string(),
            requests::Exact::Fuzzy,
        )) {
            Ok(card) => assert_eq!("Austere Command".to_string(), card.name),
            Err(e) => panic!("Failed to fetch: {:?}", e),
        }
    }
}
