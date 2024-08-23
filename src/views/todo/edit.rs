 use actix_web::{web, HttpResponse};
 use serde_json::value::Value;
 use serde_json::Map;

 use crate::state::read_file;

 use crate::todo_app::{todo_factory, enums::TaskStatus};
 use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};
 use crate::processes::process_input;
 use crate::jwt::JwToken;

 pub async fn edit(todo_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
     println!("Here is a message in the token: {}", token.message);
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

     if &status.stringify() == &TaskStatus::from_string(todo_item.status.to_string()).stringify() {
         return HttpResponse::Ok().json(ToDoItems::get_state());
     }

     process_input(existing_item, "edit".to_string(), &state);
     return HttpResponse::Ok().json(ToDoItems::get_state());
 }
