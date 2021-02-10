use actix_web::{App, HttpServer};
use schema::Config;
mod graphql;
mod model;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config {
        base: "********".to_owned(),
        client: reqwest::Client::new(),
    };
    let schema = std::sync::Arc::new(schema::create_schema());
    println!("{:?}", "entry".to_owned());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .data(schema.clone())
            .configure(graphql::route)
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run();

    eprintln!("Listening on 0.0.0.0:8080");

    server.await
}
