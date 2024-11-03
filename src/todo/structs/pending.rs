use super::super::enums::TaskStatus;
use super::base::Base;
use crate::todo::traits::create::Create;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Pending,
        };
        Pending { super_struct: base }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
