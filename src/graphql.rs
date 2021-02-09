use actix_web::{Error, HttpRequest, HttpResponse, error, web};
use crate::{schema::{Config, Context, Schema}};
use juniper::{http::GraphQLRequest};
use juniper::http::playground::playground_source;
use std::sync::Arc;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/graphql")
            .route(web::post().to(graphql))
            .route(web::get().to(playground)),
    );
}

pub async fn graphql(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    gql_req: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let config = req.app_data::<Config>().expect("config fail").to_owned();
    let ctx = Context {
        client: config.client,
        base: config.base
    };

    
    // Execute
    let res = gql_req.execute(&schema, &ctx).await;
    
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub fn playground() -> HttpResponse {
    // I prefer playground but you can use graphiql as well
    let html = playground_source("http://127.0.0.1:8000/graphql", Option::from(""));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
