use super::super::enums::TaskStatus;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(task: &str) -> Self {
        let base = Base {
            title: task.to_string(),
            status: TaskStatus::DONE,
        };
        Self { super_struct: base }
    }
}

impl Delete for Done {}
impl Get for Done {}
impl Edit for Done {}
