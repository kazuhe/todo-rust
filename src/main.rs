mod todo;

use todo::enums::TaskStatus;
use todo::{todo_factory, ItemTypes};

fn main() {
    let todo_item = todo_factory("洗濯", TaskStatus::Pending);

    match todo_item {
        ItemTypes::Done(item) => {
            println!("title: {}", item.super_struct.title);
            println!("status: {}", item.super_struct.status.stringify());
        }
        ItemTypes::Pending(item) => {
            println!("title: {}", item.super_struct.title);
            println!("status: {}", item.super_struct.status.stringify());
        }
    }
}
