use std::collections::HashMap;


enum Token {
    LeftAngleBracket,
    RightAngleBracket,
}

pub struct Dom {
    pub tag: String,
    pub attributes: Option<HashMap<String, Option<String>>>,
    pub content: Option<Vec<DomChild>>,
}

pub enum DomChild {
    String(String),
    Dom(Dom),
}