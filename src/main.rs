use std::fs;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, http, HttpResponse};
use actix_cors::Cors;
use serde_derive::{Deserialize, Serialize};

// Database operations
mod database;

// Destructed query string for reads & notes 
#[derive(Deserialize, Serialize)]
pub struct QueryObject {
    id: String
}

// Destructed query string for read_pagination
#[derive(Deserialize, Serialize)]
pub struct ReadPaginationQueryObject {
    current_page: String,
    page_size: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    database::database_init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(cors_config())
            .service(web::resource("/ping").to(ping))
            .service(web::resource("/read").to(read))
            .service(web::resource("/about").to(about))
            .service(web::resource("/pagination/read").to(pagination_read))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

// Configure of Cross-Origin Resource Sharing
fn cors_config() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

// Path: /ping
async fn ping() -> &'static str {
    "Ok"
}

// Path: /read; Query: id: String
async fn read(req: HttpRequest) -> HttpResponse {
    println!("REQ: {req:?}");
    let query_object: QueryObject = serde_qs::from_str(req.query_string()).unwrap();
    // 查询数据库结果，返回。
    match database::database_get_read(query_object) {
        Ok(result) => {
            return HttpResponse::Ok()
            .content_type("application/json")
            .insert_header(("charset", "utf-8"))
            .body(result);
        },
        
        Err(e) => {
            return HttpResponse::NotFound()
            .content_type("text/plain")
            .insert_header(("charset", "utf-8"))
            .body(e.to_string());
        },
    }

}

// Path: /about;
async fn about(req: HttpRequest) -> HttpResponse {
    println!("REQ: {req:?}");
    match fs::read_to_string("about.md") {
        Ok(result) => {
            return HttpResponse::Ok()
            .content_type("text/plain")
            .insert_header(("charset", "utf-8"))
            .body(result);
        },
        Err(e) => {
            return HttpResponse::NotFound()
            .content_type("text/plain")
            .insert_header(("charset", "utf-8"))
            .body(e.to_string());
        },
    }
}

// Path: /pagination/read; Query: current_page: String, page_size: String
async fn pagination_read(req: HttpRequest) -> HttpResponse {
    let query_object: ReadPaginationQueryObject = serde_qs::from_str(req.query_string()).unwrap();
    // 查询数据库结果，返回。
    match database::database_get_read_pagination(query_object) {
        Ok(result) => {
            return HttpResponse::Ok()
            .content_type("application/json")
            .insert_header(("charset", "utf-8"))
            .body(result);
        },
        Err(e) => {
            return HttpResponse::NotFound()
            .content_type("text/plain")
            .insert_header(("charset", "utf-8"))
            .body(e.to_string());
        },
    }
}