use std::io;

use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use log::error;
use schema::Query;

mod config;
mod model;
mod routes;
mod schema;
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load config
    let context = model::ContextStruct {
        base_url: "http:://api.dream11.com".to_owned(),
        client: actix_web::client::Client::new(),
    };

    // Start http server
    HttpServer::new(move || {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(context.clone())
            .finish();

        App::new()
            .data(context.clone())
            .data(schema)
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(routes::graphql),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(routes::graphiql),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
