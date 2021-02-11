use actix_web::HttpResponse;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
