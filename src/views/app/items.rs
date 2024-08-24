use actix_web::HttpResponse;
use super::content_loader::{read_file, add_component};

pub async fn items() -> HttpResponse {
    let mut html_content = read_file("./templates/main.html");
    let js_data = read_file("./js/main.js");
    let base_css = read_file("./css/base.css");
    let main_css = read_file("./css/main.css");

    html_content = html_content.replace("{{JAVASCRIPT}}", &js_data);
    html_content = html_content.replace("{{CSS}}", &main_css);
    html_content = html_content.replace("{{BASE_CSS}}", &base_css);
    html_content = add_component(String::from("header"), html_content);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}
