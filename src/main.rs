use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
}

struct EasyFN {
    items: Vec<CosmeticItem>,
}

impl EasyFN {
    fn new() -> Self {
        EasyFN { items: Vec::new() }
    }

    fn load_items(&mut self) {
        let data = fs::read_to_string("items.json").expect("Unable to read items file");
        self.items = serde_json::from_str(&data).expect("JSON was not well-formatted");
    }

    fn unlock_item(&self, item_name: &str) {
        let command = format!("unlock_item {}", item_name);
        Command::new("fortnite_launcher")
            .arg(command)
            .output()
            .expect("Failed to execute command");
    }

    fn unlock_all(&self) {
        for item in &self.items {
            self.unlock_item(&item.name);
        }
    }
}

fn main() {
    let mut easy_fn = EasyFN::new();
    easy_fn.load_items();
    easy_fn.unlock_all();
}