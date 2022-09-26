use actix_web::{middleware, App, HttpServer, http};
use actix_cors::Cors;

// Modules: database interaction, handlers for http server, and someting like Java bean.
mod database;
mod handler;
mod bean;

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
            .service(handler::ping)
            .service(handler::about)
            .service(handler::read)
            .service(handler::read_pagination)
    })
    .bind(("0.0.0.0", 8090))?
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
