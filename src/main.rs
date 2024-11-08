mod state;
mod todo;

use serde_json::json;
use state::{read_file, write_to_file};
use std::env;
use todo::enums::TaskStatus;
use todo::traits::delete::Delete;
use todo::traits::edit::Edit;
use todo::traits::get::Get;
use todo::{todo_factory, ItemTypes};

fn main() {
    let args: Vec<String> = env::args().collect();
    let status = &args[1];
    let title = &args[2];
    let mut state = read_file("./state.json");
    println!("変更前の operation: {:?}", state);

    state.insert(title.to_string(), json!(status));
    println!("変更後の operation: {:?}", state);

    write_to_file("./state.json", &mut state);

    let todo_item = todo_factory("洗濯", TaskStatus::Pending);

    match todo_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}
