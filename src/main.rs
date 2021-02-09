use actix_web::{App, HttpServer};
use schema::Config;
mod schema;
mod graphql;
mod model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config {
        base: "http://api.dream11.local".to_owned(),
        client: reqwest::Client::new()
    };
    let schema = std::sync::Arc::new(schema::create_schema());
    let server = HttpServer::new(move || App::new().app_data(config.clone()).data(schema.clone()).configure(graphql::route)).bind(("127.0.0.1", 8000))
        .unwrap()
        .run();

    eprintln!("Listening on 127.0.0.1:8000");

    server.await
}
