use actix_web::Responder;
use crate::json_serialization::todo_items::ToDoItems;

pub async fn get() -> impl Responder {
   ToDoItems::get_state() 
}
