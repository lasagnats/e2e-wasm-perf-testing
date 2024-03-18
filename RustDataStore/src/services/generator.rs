use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub label: String,
}


pub fn generate_entries(count: u32) -> Vec<Entry> {
    let adjectives = &[
        "pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain",
        "quaint", "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd",
        "unsightly", "adorable", "important", "inexpensive", "cheap", "expensive", "fancy"
    ];
    let colors = &["red", "yellow", "blue", "green", "pink", "brown", "purple", "brown",
                   "white", "black", "orange"];
    let nouns = &["table", "chair", "house", "bbq", "desk", "car", "pony", "cookie",
                  "sandwich", "burger", "pizza", "mouse", "keyboard"];

    let mut entries = Vec::with_capacity(count as usize);
    for id in 1..=count {
        let label = format!(
            "{} {} {}",
            adjectives.choose(&mut rand::thread_rng()).unwrap(),
            colors.choose(&mut rand::thread_rng()).unwrap(),
            nouns.choose(&mut rand::thread_rng()).unwrap()
        );

        entries.push(Entry {
            id: id.to_string(),
            label,
        });
    }

    entries
}
