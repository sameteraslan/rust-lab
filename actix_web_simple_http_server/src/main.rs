use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use std::fs;

// Define a handler for the route `/hello/{name}`
// This function extracts the `name` from the URL path and returns a greeting message
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// Define a handler for the root route `/`
// This function reads an HTML file from the `html` directory and serves it
// If the file is not found, it serves a 404 HTML message
pub async fn index(req: HttpRequest) -> impl Responder {
    println!("REQ: {req:?}");
    let html_content = fs::read_to_string("html/index.html").unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(html_content)
}

// The main function initializes the HTTP server and defines the routes
// The server listens on 127.0.0.1:8080
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
