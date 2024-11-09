use serde_json::{json, Map, Value};

use crate::{state::write_to_file, todo::enums::TaskStatus};

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Done.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} を Done に設定中..\n\n", title)
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::Pending.stringify()));
        write_to_file("./state.json", state);
        println!("\n\n{} を Pending に設定中..\n\n", title)
    }
}
