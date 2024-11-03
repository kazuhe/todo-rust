pub mod enums;
pub mod structs;

use enums::TaskStatus;
use structs::{done::Done, pending::Pending};

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::Done => ItemTypes::Done(Done::new(title)),
        TaskStatus::Pending => ItemTypes::Pending(Pending::new(title)),
    }
}
