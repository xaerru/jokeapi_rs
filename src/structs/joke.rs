use serde::{Deserialize, Serialize};

/// JSON result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub error: bool,
    pub category: String,
    #[serde(flatten)]
    pub kind: DataKind,
    pub flags: Flags,
    pub id: i64,
    pub safe: bool,
    pub lang: String,
}

/// Blacklist Flags
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Flags {
    pub nsfw: bool,
    pub religious: bool,
    pub political: bool,
    pub racist: bool,
    pub sexist: bool,
    pub explicit: bool,
}

/// Kind of joke
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DataKind {
    TwoPart { setup: String, delivery: String },
    Single { joke: String },
}
