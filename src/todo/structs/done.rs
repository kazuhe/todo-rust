use super::super::enums::TaskStatus;
use super::base::Base;
use crate::todo::traits::delete::Delete;
use crate::todo::traits::edit::Edit;
use crate::todo::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Done,
        };
        Done { super_struct: base }
    }
}

impl Get for Done {}

impl Delete for Done {}

impl Edit for Done {}
