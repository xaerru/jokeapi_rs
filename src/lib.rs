//! # jokeapi_rs
//!
//! Rust wrapper for [JokeAPI](https://v2.jokeapi.dev/)
//!
//! ## Get a joke
//!
//! ```rust
//! use jokeapi_rs::Joke;
//!
//! fn main() {
//!     println!("{}", Joke::new().fetch().joke());
//! }
//! ```
//!
//! ## Get a joke of a certain type
//!
//! Type can either be "single" or "twopart"
//!
//! ```rust
//! use jokeapi_rs::Joke;
//!
//! fn main() {
//!     println!("{}", Joke::new().of_type("single").fetch().joke());
//! }
//! ```
//!
//! ## Get a joke which belongs to certain categories
//!
//! ```rust
//! use jokeapi_rs::Joke;
//!
//! fn main() {
//!     println!(
//!         "{}",
//!         Joke::new()
//!             .categories(
//!                 ["programming", "spooky"]
//!                     .iter()
//!                     .map(|x| x.to_string())
//!                     .collect()
//!             )
//!             .fetch()
//!             .joke()
//!     )
//! }
//! ```
//!
//! ## Get a Data struct which contains all the fields
//!
//! ```rust
//! use jokeapi_rs::structs::joke::Data;
//! use jokeapi_rs::structs::joke::DataKind;
//! use jokeapi_rs::Joke;
//!
//! fn main() {
//!     let res: Data = Joke::new()
//!         .of_type("single")
//!         .categories(
//!             ["programming", "pun"]
//!                 .iter()
//!                 .map(|x| x.to_string())
//!                 .collect(),
//!         )
//!         .blacklist(["sexist", "nsfw"].iter().map(|x| x.to_string()).collect())
//!         //.safe() // Uncomment to enable safe mode
//!         .fetch();
//!
//!     // Get the joke
//!     // .joke() method on res does the following for you
//!     match res.kind.clone() {
//!         DataKind::TwoPart { setup, delivery } => {
//!             println!("{}\n{}", setup, delivery)
//!         }
//!         DataKind::Single { joke } => println!("{}", joke),
//!     }
//!
//!     // Access all the json fields
//!     println!("{:#?}", res);
//! }
//! ```

/// Struct definitions
pub mod structs;

use structs::joke::Data;
use structs::joke::DataKind;

/// Main joke struct
pub struct Joke {
    pub url: String,
    pub joke: String,
    pub joke_type: String,
    pub categories: String,
    pub blacklist_flags: String,
    pub safe: String,
    client: reqwest::Client,
}

impl Data {
    /// Get the joke
    pub fn joke(&mut self) -> String {
        match self.kind.clone() {
            DataKind::Single { joke } => joke,
            DataKind::TwoPart { setup, delivery } => format!("{}\n{}", setup, delivery),
        }
    }
}

impl Joke {
    /// Initialize the Joke struct.
    pub fn new() -> Joke {
        Joke {
            url: String::from("https://v2.jokeapi.dev/joke/"),
            joke: String::new(),
            joke_type: String::new(),
            categories: String::from("Any"),
            blacklist_flags: String::new(),
            safe: String::new(),
            client: reqwest::Client::new(),
        }
    }

    /// Fetch the Data from JokeAPI.
    #[tokio::main]
    pub async fn fetch(&mut self) -> Data {
        let res: Data = self
            .client
            .get(format!(
                "{}{}?{}&{}&{}",
                self.url, self.categories, self.joke_type, self.blacklist_flags, self.safe
            ))
            .send()
            .await
            .expect("Couldn't get the URL")
            .json()
            .await
            .expect("Couldn't parse the data");
        res
    }

    /// Specify the type of the joke (single or twopart).
    /// If not specified joke could be of any type.
    pub fn of_type(&mut self, joke_type: &str) -> &mut Self {
        match joke_type.to_lowercase().as_str() {
            "single" => self.joke_type = String::from("type=single"),
            "twopart" => self.joke_type = String::from("type=twopart"),
            _ => panic!("Invalid joke type. Joke type can be either \"single\" or \"twopart\""),
        }
        self
    }

    /// Specify certain categories the joke should fit in.
    /// Available categories:
    /// - Programming
    /// - Misc
    /// - Dar
    /// - Pun
    /// - Spooky
    /// - Christmas
    pub fn categories(&mut self, categories: Vec<String>) -> &mut Self {
        let all_categories: Vec<String> =
            ["programming", "misc", "dark", "pun", "spooky", "christmas"]
                .iter()
                .map(|x| x.to_string())
                .collect();
        let categories: Vec<String> = categories.iter().map(|x| x.to_lowercase()).collect();
        if categories.iter().any(|x| all_categories.contains(x)) {
            self.categories = categories.join(",");
            self
        } else {
            panic!("Invalid categories")
        }
    }

    /// Flags to blacklist.
    /// Available flags:
    /// - nsfw
    /// - religious
    /// - political
    /// - racist
    /// - sexist
    /// - explicit
    pub fn blacklist(&mut self, blacklist_flags: Vec<String>) -> &mut Self {
        let all_flags: Vec<String> = [
            "nsfw",
            "religious",
            "political",
            "racist",
            "sexist",
            "explicit",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let blacklist_flags: Vec<String> =
            blacklist_flags.iter().map(|x| x.to_lowercase()).collect();
        if blacklist_flags.iter().any(|x| all_flags.contains(x)) {
            self.blacklist_flags = format!("blacklistFlags={}", blacklist_flags.join(","));
            self
        } else {
            panic!("Invalid flags")
        }
    }

    /// Enable safe mode.
    pub fn safe(&mut self) -> &mut Self {
        self.safe = String::from("safe-mode");
        self
    }
}
