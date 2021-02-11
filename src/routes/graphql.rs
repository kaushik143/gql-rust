use actix_web::{web, HttpRequest};
use async_graphql_actix_web::{Request, Response};

use crate::schema::AppSchema;

pub async fn graphql(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    // let token = Token::from_request(&req, &mut actix_web::dev::Payload::None)
    //     .await
    //     .ok();

    let mut request = gql_request.into_inner();
    // request = request.data(token);
    schema.execute(request).await.into()
}
