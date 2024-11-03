mod todo;

use todo::enums::TaskStatus;
use todo::traits::delete::Delete;
use todo::traits::edit::Edit;
use todo::traits::get::Get;
use todo::{todo_factory, ItemTypes};

fn main() {
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
