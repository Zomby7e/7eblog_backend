/**
 * Handlers for HttpServer
 */

use std::fs;
// Maybe add "post" and "delete" in the future.
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use crate::database;
use crate::bean::{ReadIDQuery, ReadPaginationQuery};


// ping this server.
#[get("/ping")]
async fn ping() -> &'static str {
    "7eBlog - Powered by Actix-web."
}

#[get("/about")]
async fn about() -> HttpResponse {
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

#[get("/read")]
async fn read(req: HttpRequest) -> impl Responder {
    let query_object: ReadIDQuery = serde_qs::from_str(req.query_string()).unwrap();
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

#[get("/pagination/read")]
async fn read_pagination(req: HttpRequest) -> HttpResponse {
    let query_object: ReadPaginationQuery = serde_qs::from_str(req.query_string()).unwrap();
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