use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use query::Query;
mod query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
