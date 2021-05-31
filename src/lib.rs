pub mod structs;

use structs::joke::Data;
use structs::joke::DataKind;

pub struct Joke {
    pub url: String,
    pub joke: String,
    pub joke_type: String,
    pub categories: String,
    pub blacklist_flags: String,
    client: reqwest::Client,
}

impl Data {
    pub fn joke(&mut self) -> String {
        match self.kind.clone() {
            DataKind::Single { joke } => joke,
            DataKind::TwoPart { setup, delivery } => format!("{}\n{}", setup, delivery),
        }
    }
}

impl Joke {
    pub fn new() -> Joke {
        Joke {
            url: String::from("https://v2.jokeapi.dev/joke/"),
            joke: String::new(),
            joke_type: String::new(),
            categories: String::from("Any"),
            blacklist_flags: String::new(),
            client: reqwest::Client::new(),
        }
    }

    #[tokio::main]
    pub async fn fetch(&mut self) -> Data {
        let res: Data = self
            .client
            .get(format!(
                "{}{}?{}&{}",
                self.url, self.categories, self.joke_type, self.blacklist_flags
            ))
            .send()
            .await
            .expect("Couldn't get the URL")
            .json()
            .await
            .expect("Couldn't parse the data");
        res
    }

    pub fn of_type(&mut self, joke_type: &str) -> &mut Self {
        match joke_type.to_lowercase().as_str() {
            "single" => self.joke_type = String::from("type=single"),
            "twopart" => self.joke_type = String::from("type=twopart"),
            _ => panic!("Invalid joke type"),
        }
        self
    }

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
}
