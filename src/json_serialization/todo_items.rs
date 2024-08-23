use crate::todo_app::ItemTypes;
use crate::todo_app::structs::base::Base;
use crate::state::read_file;
use crate::todo_app::{todo_factory, enums::TaskStatus};

use serde_json::value::Value;
use serde_json::Map;
use serde::Serialize;
use actix_web:: {
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut done_buffer = Vec::new();
        let mut pending_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Done(done) => done_buffer.push(done.super_struct),
                ItemTypes::Pending(pending) => pending_buffer.push(pending.super_struct),
            }
        }
        let pending_count = pending_buffer.len() as i8;
        let done_count = done_buffer.len() as i8;

        Self {
            pending_items: pending_buffer,
            done_items: done_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        let state: Map<String, Value> = read_file("./state.json");
        let mut array_buffer = Vec::new();

        for (key, value) in state {
            let status: TaskStatus = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item: ItemTypes = todo_factory(&key, status);
            array_buffer.push(item);
        }
        ToDoItems::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;
    fn respond_to(self, _req:&HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
