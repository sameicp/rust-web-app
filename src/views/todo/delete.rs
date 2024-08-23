use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::todo_app::{todo_factory, enums::TaskStatus};
use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};
use crate::jwt::JwToken;
use crate::processes::process_input;
use crate::state::read_file;

pub async fn delete(todo_item: web::Json<ToDoItem>, _token: JwToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
     match &state.get(&todo_item.title) {
         Some(result) => {
             status = TaskStatus::from_string(result.as_str().unwrap().to_owned());
         }
         None => {
             return HttpResponse::NotFound().json(format!("{} not in state", &todo_item.title));
         }
     }

    let existing_item = todo_factory(todo_item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
    
}

