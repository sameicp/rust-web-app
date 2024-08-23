use actix_web::HttpResponse;
use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let mut html_content = read_file("./templates/main.html");
    let js_data = read_file("./js/main.js");

    html_content = html_content.replace("{{JAVASCRIPT}}", &js_data);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}
