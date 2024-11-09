use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}, \nStatus: {}\n\n", title, result);
            }
            None => {
                println!("Item: {} は存在しません", title);
            }
        }
    }
}
