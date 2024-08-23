use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::todo_app::{todo_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    println!("create function called");
    let state: Map<String, Value> = read_file("./state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();
    println!("Finished formating title");
    let item = todo_factory(&title.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    println!("Finished processing the input");
    format!("{} created", title)
}

